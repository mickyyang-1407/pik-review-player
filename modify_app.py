import re

with open("src/App.tsx", "r") as f:
    content = f.read()

# Add imports
content = content.replace("import { notes, addNote, updateNoteStatus, deleteNote, timecodedNotes } from './stores/notesStore';", "import { notes, addNote, updateNoteStatus, deleteNote, timecodedNotes, loadNotes, Project, Version } from './stores/notesStore';")

# Add state
state_code = """
  const [currentProject, setCurrentProject] = createSignal<Project | null>(null);
  const [currentVersion, setCurrentVersion] = createSignal<Version | null>(null);
  const [versions, setVersions] = createSignal<Version[]>([]);

  const currentTrack = () => tracks().find((track) => track.path === filePath()) ?? null;
"""
content = content.replace("  const currentTrack = () => tracks().find((track) => track.path === filePath()) ?? null;", state_code)

# Modify loadFile
load_file_code = """
  async function loadFile(path: string, autoPlay: boolean = false) {
    setError('');
    setStatus('loading');
    try {
      await invoke('playback_load', { path });
      refreshLibrary();

      // Project & Version Logic
      const fileName = path.split(/[\\\\/]/).pop() || 'Untitled';
      const fileStem = fileName.replace(/\.[^/.]+$/, "");

      const allProjects = await invoke<Project[]>('review_list_projects');
      let proj = allProjects.find(p => p.title === fileStem);
      if (!proj) {
        proj = await invoke<Project>('review_create_project', { title: fileStem });
      }
      setCurrentProject(proj);

      const existingVersions = await invoke<Version[]>('review_get_versions', { projectId: proj.id });
      // Check if this file is already a version
      let version = existingVersions.find(v => v.filePath === path);
      if (!version) {
        const versionLabel = `V${existingVersions.length + 1}`;
        version = await invoke<Version>('review_create_version', { 
          projectId: proj.id, 
          label: versionLabel, 
          filePath: path 
        });
      }
      
      const updatedVersions = await invoke<Version[]>('review_get_versions', { projectId: proj.id });
      setVersions(updatedVersions);
      setCurrentVersion(version);

      await loadNotes(proj.id);

      if (autoPlay) {
        await invoke('playback_play');
      }
    } catch (e: unknown) {
      setError(String(e));
      setStatus('error');
    }
  }

  async function createNewVersion() {
    const proj = currentProject();
    if (!proj || !filePath()) return;
    try {
      const existingVersions = await invoke<Version[]>('review_get_versions', { projectId: proj.id });
      const versionLabel = `V${existingVersions.length + 1}`;
      const version = await invoke<Version>('review_create_version', { 
        projectId: proj.id, 
        label: versionLabel, 
        filePath: filePath()
      });
      const updatedVersions = await invoke<Version[]>('review_get_versions', { projectId: proj.id });
      setVersions(updatedVersions);
      setCurrentVersion(version);
    } catch (e) {
      console.warn('Failed to create new version', e);
    }
  }
"""

content = re.sub(r'  async function loadFile\(path: string, autoPlay: boolean = false\) \{.*?(?=\n  function togglePlay\(\))', load_file_code.strip(), content, flags=re.DOTALL)

# Modify handleKeyDown to use currentProject and currentVersion
content = content.replace("if (nearby) updateNoteStatus(nearby.id, 'done');", "if (nearby) updateNoteStatus(nearby.id as any, 'done');")

# Pass props to NotesPanel
notes_panel_code = """
          <NotesPanel 
            notes={notes()}
            currentPosition={position()}
            onAddNote={(body, ms, role) => {
              if (currentProject()) {
                addNote(currentProject()!.id, currentVersion()?.id || null, body, ms, role);
              }
            }}
            onUpdateStatus={(id, status) => updateNoteStatus(id as any, status)}
            onDeleteNote={(id) => deleteNote(id as any)}
            onSeekToNote={(ms) => onSeek(ms / 1000)}
            currentProject={currentProject()}
            currentVersion={currentVersion()}
            versions={versions()}
            onVersionSelect={(v) => setCurrentVersion(v)}
            onCreateVersion={createNewVersion}
          />
"""
content = re.sub(r'<NotesPanel[^>]+/>', notes_panel_code.strip(), content)

with open("src/App.tsx", "w") as f:
    f.write(content)
