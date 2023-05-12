// main.rs

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Return type of custom command; must implement Clone & serde::Serialize
#[derive(Clone, serde::Serialize)]
struct Payload {
  result: bool, // Bool encoding whether all operations succeeded or not
  succ_log: String, // String logging which operations succeeded
  fail_log: String // String logging which operations failed
}

// Function that helps the custom command perform file operations
// Returns a bool indicating if the operation succeeded
fn fs_helper(src_path: &str, dst_path: &str, overwrite: bool) -> bool {
  let mut options: fs_extra::dir::CopyOptions = fs_extra::dir::CopyOptions::new();
  options.overwrite = overwrite;
  options.skip_exist = !overwrite;
  let mut from: Vec<&str> = Vec::new();
  from.push(src_path);
  match fs_extra::copy_items(&from, dst_path, &options) {
    Ok(_i) => return true,
    Err(_e) => return false
  };
}

// Custom command that handles form submission data and the corresponding file operations
// To see arguments, look at renderer.js
#[tauri::command]
fn handle_form_submit(arr_labs_selected: [bool; 5], arr_labs_systems: [u32; 5], src_path: &str, dst_path: &str, overwrite: bool) -> Payload {
  let mut res: bool = true;
  let mut success_log_str: String = String::from("");
  let mut failure_log_str: String = String::from("");

  // // Testing code:
  // let rc: bool = fs_helper(src_path, dst_path, false);
  // if !rc {
  //   res = false;
  //   failure_log_str.push_str("test code");
  // } else {
  //   success_log_str.push_str("test code");
  // }

  let template_right: String = ".mcc.northwestern.edu\\".to_string();

  // Bodeen
  if arr_labs_selected[0] {
    // Decode bits to see which systems to do
    let mut bodeen_systems_todo: Vec<i32> = Vec::new();
    let bodeen_systems: u32 = arr_labs_systems[0];
    for i in 0..5 {
      if (bodeen_systems >> i) & 1 != 0 {
        bodeen_systems_todo.push(i32::try_from(i + 1).unwrap());
      }
    }
    if bodeen_systems_todo.is_empty() {
      bodeen_systems_todo = vec![1, 2, 3, 4, 5];
    }

    let bodeen_template_left: String = "\\\\bodeen-0".to_string();

    // Copy files to selected systems
    for i in bodeen_systems_todo {
      let i_str: String = i.to_string();
      let combined_bodeen_dst_path: String = format!("{}{}{}{}", bodeen_template_left, i_str, template_right, dst_path);

      let rc: bool = fs_helper(src_path, combined_bodeen_dst_path.as_str(), overwrite);

      if !rc {
        res = false;
        failure_log_str.push_str("bodeen-0");
        failure_log_str.push_str(&i_str);
        failure_log_str.push_str("<br>");
      } else {
        success_log_str.push_str("bodeen-0");
        success_log_str.push_str(&i_str);
        success_log_str.push_str("<br>");
      }
    }
  }

  // MSE
  if arr_labs_selected[1] {
    // Decode bits to see which systems to do
    let mut mse_systems_todo: Vec<i32> = Vec::new();
    let mse_systems: u32 = arr_labs_systems[1];
    for i in 0..7 {
      if (mse_systems >> i) & 1 != 0 {
        mse_systems_todo.push(i32::try_from(i + 1).unwrap());
      }
    }
    if mse_systems_todo.is_empty() {
      mse_systems_todo = vec![1, 2, 3, 4, 5, 6, 7];
    }

    let mse_template_left: String = "\\\\mse-0".to_string();

    // Copy files to selected systems
    for i in mse_systems_todo {
      let i_str: String = i.to_string();
      let combined_mse_dst_path: String = format!("{}{}{}{}", mse_template_left, i_str, template_right, dst_path);

      let rc: bool = fs_helper(src_path, combined_mse_dst_path.as_str(), overwrite);

      if !rc {
        res = false;
        failure_log_str.push_str("mse-0");
        failure_log_str.push_str(&i_str);
        failure_log_str.push_str("<br>");
      } else {
        success_log_str.push_str("mse-0");
        success_log_str.push_str(&i_str);
        success_log_str.push_str("<br>");
      }
    }
  }

  // ChBe
  if arr_labs_selected[2] {
    // Decode bits to see which systems to do
    let mut chbe_systems_todo: Vec<i32> = Vec::new();
    let chbe_systems: u32 = arr_labs_systems[2];
    for i in 0..8 {
      if (chbe_systems >> i) & 1 != 0 {
        chbe_systems_todo.push(i32::try_from(i + 1).unwrap());
      }
    }
    if chbe_systems_todo.is_empty() {
      chbe_systems_todo = vec![1, 2, 3, 4, 5, 6, 7, 8];
    }

    let chbe_template_left: String = "\\\\e1-chbe-0".to_string();

    // Copy files to selected systems
    for i in chbe_systems_todo {
      let i_str: String = i.to_string();
      let combined_chbe_dst_path: String = format!("{}{}{}{}", chbe_template_left, i_str, template_right, dst_path);

      let rc: bool = fs_helper(src_path, combined_chbe_dst_path.as_str(), overwrite);

      if !rc {
        res = false;
        failure_log_str.push_str("e1-chbe-0");
        failure_log_str.push_str(&i_str);
        failure_log_str.push_str("<br>");
      } else {
        success_log_str.push_str("e1-chbe-0");
        success_log_str.push_str(&i_str);
        success_log_str.push_str("<br>");
      }
    }
  }

  // Segal
  if arr_labs_selected[3] {
    // Decode bits to see which systems to do
    let mut segal_systems_todo: Vec<i32> = Vec::new();
    let segal_systems: u32 = arr_labs_systems[3];
    for i in 0..7 {
      if (segal_systems >> i) & 1 != 0 {
        segal_systems_todo.push(i32::try_from(i + 1).unwrap());
      }
    }
    if segal_systems_todo.is_empty() {
      segal_systems_todo = vec![1, 2, 3, 4, 5, 6, 7];
    }

    let segal_template_left: String = "\\\\e1-segal-0".to_string();

    // Copy files to selected systems
    for i in segal_systems_todo {
      let i_str: String = i.to_string();
      let combined_segal_dst_path: String = format!("{}{}{}{}", segal_template_left, i_str, template_right, dst_path);

      let rc: bool = fs_helper(src_path, combined_segal_dst_path.as_str(), overwrite);

      if !rc {
        res = false;
        failure_log_str.push_str("e1-segal-0");
        failure_log_str.push_str(&i_str);
        failure_log_str.push_str("<br>");
      } else {
        success_log_str.push_str("e1-segal-0");
        success_log_str.push_str(&i_str);
        success_log_str.push_str("<br>");
      }
    }
  }

  // MCC
  if arr_labs_selected[4] {
    // Decode bits to see which systems to do
    let mut mcc_systems_todo: Vec<i32> = Vec::new();
    let mcc_systems: u32 = arr_labs_systems[4];
    for i in 0..26 {
      if (mcc_systems >> i) & 1 != 0 {
        mcc_systems_todo.push(i32::try_from(i + 1).unwrap());
      }
    }
    if mcc_systems_todo.is_empty() {
      mcc_systems_todo = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26];
    }

    let mcc_template_left: String = "\\\\e1-mcc-".to_string();

    // Copy files to selected systems
    for i in mcc_systems_todo {
      let mut i_str: String = i.to_string();
      if i <= 9 {
        i_str = format!("{}{}", 0, i_str);
      }
      let combined_mcc_dst_path: String = format!("{}{}{}{}", mcc_template_left, i_str, template_right, dst_path);

      let rc: bool = fs_helper(src_path, combined_mcc_dst_path.as_str(), overwrite);

      if !rc {
        res = false;
        failure_log_str.push_str("e1-mcc-");
        failure_log_str.push_str(&i_str);
        failure_log_str.push_str("<br>");
      } else {
        success_log_str.push_str("e1-mcc-");
        success_log_str.push_str(&i_str);
        success_log_str.push_str("<br>");
      }
    }
  }

  return Payload {
    result: res,
    succ_log: success_log_str,
    fail_log: failure_log_str
  };
}

// A handler must be generated for every custom command
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![handle_form_submit])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
