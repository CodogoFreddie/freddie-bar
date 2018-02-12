use std::process::Command;

use serde_json;
use render;

#[derive(Serialize, Deserialize, Debug)]
struct Rect {
    x: i64,
    y: i64,
    width: i64,
    height: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Workspace {
    num: i64,
    name: String,
    visible: bool,
    focused: bool,
    rect: Rect,
    output: String,
    urgent: bool,
}

fn render_workspace (workspace : Workspace ) -> String {
    let Workspace { urgent, visible, focused, name, .. } = workspace;

    let wide_name = format!(" {} ", name);

    if urgent {
        return render::with_bg(render::ORANGE, wide_name);
    }

    if visible {
        return render::with_bg(render::RED, wide_name);
    }

    if focused {
        return render::with_bg(render::BLUE, wide_name);
    }

    return render::with_bg(render::GREEN, wide_name);
}

//i3-msg -t get_workspaces
pub fn get(screen: String) -> String {
    let output = Command::new("i3-msg")
        .args( &["-t", "get_workspaces"])
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    if !output.status.success() {
        return String::from("failure");
    }

    let command_response = String::from_utf8_lossy(&output.stdout);

    let parsed_workspaces : Vec<Workspace> = match serde_json::from_str(&*command_response) {
        Err(why) => panic!("{:?}", why),
        Ok(workspace) => workspace,
    };

    let mut rendered_workspaces = Vec::new();
    for workspace in parsed_workspaces {
        if workspace.output == screen {
            rendered_workspaces.push(render_workspace(workspace));
        }
    }

    return rendered_workspaces.join("");
}
