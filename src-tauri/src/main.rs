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
fn fs_helper(src_path: &str, dst_path: &str) -> bool {
  let options: fs_extra::dir::CopyOptions = fs_extra::dir::CopyOptions::new();
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
fn handle_form_submit(arr_labs_selected: [bool; 5], src_path: &str, dst_path: &str) -> Payload {
  // Return variables
  let mut res: bool = true;
  let mut success_log_str: String = String::from("");
  let mut failure_log_str: String = String::from("");

  // // Testing code:
  // let rc: bool = fs_helper(src_path, dst_path);
  // if !rc {
  //   res = false;
  //   failure_log_str.push_str("test code");
  // } else {
  //   success_log_str.push_str("test code");
  // }

  // Right half of template
  let template_right: String = ".mcc.northwestern.edu\\".to_string();

  // Bodeen
  if arr_labs_selected[0] {
    // Left half of template for Bodeen
    let bodeen_template_left: String = "\\\\bodeen-0".to_string();
    // Copy files to all 5 of the Bodeen systems
    for i in 1..6 {
      let i_str: String = i.to_string();
      // Put everything together to get the final destination path
      let combined_bodeen_dst_path: String = format!("{}{}{}{}", bodeen_template_left, i_str, template_right, dst_path);
      // Perform file operation
      let rc: bool = fs_helper(src_path, combined_bodeen_dst_path.as_str());
      // Update the return variables
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
    // Left half of template for MSE
    let mse_template_left: String = "\\\\mse-0".to_string();
    // Copy files to all 7 of the MSE systems
    for i in 1..8 {
      let i_str: String = i.to_string();
      // Put everything together to get the final destination path
      let combined_mse_dst_path: String = format!("{}{}{}{}", mse_template_left, i_str, template_right, dst_path);
      // Perform file operation
      let rc: bool = fs_helper(src_path, combined_mse_dst_path.as_str());
      // Update the return variables
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
    // Left half of template for ChBe
    let chbe_template_left: String = "\\\\e1-chbe-0".to_string();
    // Copy files to all 8 of the ChBe systems
    for i in 1..9 {
      let i_str: String = i.to_string();
      // Put everything together to get the final destination path
      let combined_chbe_dst_path: String = format!("{}{}{}{}", chbe_template_left, i_str, template_right, dst_path);
      // Perform file operation
      let rc: bool = fs_helper(src_path, combined_chbe_dst_path.as_str());
      // Update the return variables
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
    // Left half of template for Segal
    let segal_template_left: String = "\\\\e1-segal-0".to_string();
    // Copy files to all 7 of the Segal systems
    for i in 1..8 {
      let i_str: String = i.to_string();
      // Put everything together to get the final destination path
      let combined_segal_dst_path: String = format!("{}{}{}{}", segal_template_left, i_str, template_right, dst_path);
      // Perform file operation
      let rc: bool = fs_helper(src_path, combined_segal_dst_path.as_str());
      // Update the return variables
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
    // Left half of template for MCC
    let mcc_template_left: String = "\\\\e1-mcc-".to_string();
    // Copy files to all 26 of the MCC systems
    for i in 1..27 {
      let mut i_str: String = i.to_string();
      if i <= 9 {
        i_str = format!("{}{}", 0, i_str);
      }
      // Put everything together to get the final destination path
      let combined_mcc_dst_path: String = format!("{}{}{}{}", mcc_template_left, i_str, template_right, dst_path);
      println!("{}", combined_mcc_dst_path);
      // Perform file operation
      let rc: bool = fs_helper(src_path, combined_mcc_dst_path.as_str());
      // Update the return variables
      if !rc {
        res = false;
        failure_log_str.push_str("e1-mcc-0");
        failure_log_str.push_str(&i_str);
        failure_log_str.push_str("<br>");
      } else {
        success_log_str.push_str("e1-mcc-0");
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
