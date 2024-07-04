use zellij_tile::prelude::*;

use std::collections::BTreeMap;

const CARRIAGE_RETURN: u8 = 13;
const ESCAPE: u8 = 27;

#[derive(Default)]
struct State {
    userspace_configuration: BTreeMap<String, String>,
    pane_manifest: Option<PaneManifest>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.userspace_configuration = configuration;
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
            PermissionType::WriteToStdin,
        ]);
        subscribe(&[EventType::PaneUpdate, EventType::PermissionRequestResult]);
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if pipe_message.payload.is_none() {
            return false;
        }
        let payload = pipe_message
            .payload
            .expect("is not none and should be some");
        let name = pipe_message.name;
        if let Some(ref pane_manifest) = self.pane_manifest {
            let panes = pane_manifest.panes.get(&0);
            if let Some(panes) = panes {
                for pane in panes {
                    if pane.title == name {
                        focus_terminal_pane(pane.id, false);
                        // write(vec![ESCAPE]);
                        write_chars(&payload);
                        write(vec![CARRIAGE_RETURN]);
                        break;
                    }
                }
            }
        }
        false
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PaneUpdate(pane_manifest) => {
                self.pane_manifest = Some(pane_manifest);
            }
            Event::PermissionRequestResult(permission_status) => match permission_status {
                PermissionStatus::Granted => hide_self(),
                PermissionStatus::Denied => (),
            },
            _ => {}
        }
        false
    }
}
