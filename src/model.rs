use git2::Repository;
use std::fmt;

/// Application state containing all data needed for the Git client
pub struct Model {
    /// Current Git repository
    pub repository: Option<Repository>,
    /// Current working directory path
    pub current_path: String,
    /// Application running state
    pub running_state: RunningState,
    /// Current view/screen being displayed
    pub current_view: View,
    /// Selected item index in current view
    pub selected_index: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum View {
    #[default]
    Status,
    Log,
    Branches,
    Files,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            repository: None,
            current_path: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string()),
            running_state: RunningState::default(),
            current_view: View::default(),
            selected_index: 0,
        }
    }
}

impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Model")
            .field("repository", &self.repository.as_ref().map(|r| r.path()))
            .field("current_path", &self.current_path)
            .field("running_state", &self.running_state)
            .field("current_view", &self.current_view)
            .field("selected_index", &self.selected_index)
            .finish()
    }
}

impl Model {
    /// Initialize the model and try to open a Git repository
    pub fn new() -> Result<Self, git2::Error> {
        let mut model = Self::default();
        model.load_repository()?;
        Ok(model)
    }

    /// Load or reload the Git repository from current path
    pub fn load_repository(&mut self) -> Result<(), git2::Error> {
        match Repository::discover(&self.current_path) {
            Ok(repo) => {
                self.repository = Some(repo);
                Ok(())
            }
            Err(e) => {
                self.repository = None;
                Err(e)
            }
        }
    }

    /// Check if we have a valid Git repository
    pub fn has_repository(&self) -> bool {
        self.repository.is_some()
    }
}
