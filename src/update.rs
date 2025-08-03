use crate::model::{Model, RunningState, View};

/// Messages representing all possible actions in the application
#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    /// Quit the application
    Quit,
    /// Navigate between views
    SwitchView(View),
    /// Move selection up
    SelectUp,
    /// Move selection down
    SelectDown,
    /// Refresh current view data
    Refresh,
    /// Go to first item
    SelectFirst,
    /// Go to last item
    SelectLast,
}

/// Update function that processes messages and returns new state
/// Returns Option<Message> to allow chaining of updates
pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
        Message::SwitchView(view) => {
            model.current_view = view;
            model.selected_index = 0;
        }
        Message::SelectUp => {
            if model.selected_index > 0 {
                model.selected_index -= 1;
            }
        }
        Message::SelectDown => {
            // Note: actual bounds checking should be done in view-specific logic
            model.selected_index = model.selected_index.saturating_add(1);
        }
        Message::Refresh => {
            // Try to reload the repository
            let _ = model.load_repository();
        }
        Message::SelectFirst => {
            model.selected_index = 0;
        }
        Message::SelectLast => {
            // Note: actual max should be calculated based on current view content
            model.selected_index = usize::MAX;
        }
    }
    None // No chained messages for now
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quit_message() {
        let mut model = Model::default();
        update(&mut model, Message::Quit);
        assert_eq!(model.running_state, RunningState::Done);
    }

    #[test]
    fn test_switch_view() {
        let mut model = Model::default();
        model.selected_index = 5;
        
        update(&mut model, Message::SwitchView(View::Log));
        assert_eq!(model.current_view, View::Log);
        assert_eq!(model.selected_index, 0); // Should reset selection
    }

    #[test]
    fn test_navigation() {
        let mut model = Model::default();
        
        // Test select down
        update(&mut model, Message::SelectDown);
        assert_eq!(model.selected_index, 1);
        
        // Test select up
        update(&mut model, Message::SelectUp);
        assert_eq!(model.selected_index, 0);
        
        // Test select up at boundary
        update(&mut model, Message::SelectUp);
        assert_eq!(model.selected_index, 0); // Should not go below 0
    }
}
