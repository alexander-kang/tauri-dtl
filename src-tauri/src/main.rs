// main.rs

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Function that helps the custom command perform file operations
// Returns a bool:
//     true - success
//     false - failure
fn fs_helper(src_path: &str, dst_path: &str) -> bool {
  // Make default copy options object
  let options = fs_extra::dir::CopyOptions::new();

  // Make list to copy from (needed in order to use copy_items() from fs_extra)
  let mut from = Vec::new();
  // Add src_path to this list
  from.push(src_path);

  // Do the file operations and based on the copy_items() return value, return the corresponding integer
  match fs_extra::copy_items(&from, dst_path, &options) {
    Ok(_i) => return true,
    Err(_e) => return false
  };
}

// Custom command that handles form submission data and the corresponding file operations
// labs: array of 5 bools that represents if the corresponding lab is being worked on
//     0 - Bodeen
//     1 - MSE
//     2 - ChBe
//     3 - Segal
//     4 - MCC
// src_path: string representing the file path to copy from
// dst_path: string representing the parsed file path to copy to
// Returns a u8:
//     1st bit (LSb) - test code failure
//     2nd bit - Bodeen failure
//     3rd bit - MSE failure
//     4th bit - ChBe failure
//     5th bit - Segal failure
//     6th bit - MCC failure
//     If the value is 0, it means success
#[tauri::command]
fn handle_form_submit(arr_labs: [bool; 5], src_path: &str, dst_path: &str) -> u8 {
  // Return value
  let mut ret: u8 = 0;

  // // Testing code:
  // let rc = fs_helper(src_path, "put_testing_dst_here");
  // if rc {
  //   return 0;
  // } else {
  //   return 1;
  // }

  // Right half of template
  let template_right: String = ".mcc.northwestern.edu\\".to_string();

  // Check if Bodeen is being worked on
  if arr_labs[0] {
    // Variable that lets us know if there was a failure in executing a file operation
    let mut failure: bool = false;
    // Left half of template for Bodeen
    let bodeen_template_left: String = "\\\\bodeen-0".to_string();
    // Copy files to all 5 of the Bodeen systems
    for i in 1..6 {
      let i_str: String = i.to_string();
      // Put everything together to get the final destination path
      let combined_bodeen_dst_path = format!("{}{}{}{}", bodeen_template_left, i_str, template_right, dst_path);
      // Perform file operation
      let rc = fs_helper(src_path, combined_bodeen_dst_path.as_str());
      // If the file operation failed, update the failure variable
      if !rc {
        failure = true;
      }
    }
    // If any of the 5 file operations failed, change the return value to represent that
    if failure {
      ret += 0b10;
    }
  }

  // Check if MSE is being worked on
  if arr_labs[1] {
    // Variable that lets us know if there was a failure in executing a file operation
    let mut failure: bool = false;
    // Left half of template for MSE
    let mse_template_left: String = "\\\\mse-0".to_string();
    // Copy files to all 7 of the MSE systems
    for i in 1..8 {
      let i_str: String = i.to_string();
      // Put everything together to get the final destination path
      let combined_mse_dst_path = format!("{}{}{}{}", mse_template_left, i_str, template_right, dst_path);
      // Perform file operation
      let rc = fs_helper(src_path, combined_mse_dst_path.as_str());
      // If the file operation failed, update the failure variable
      if !rc {
        failure = true;
      }
    }
    // If any of the 7 file operations failed, change the return value to represent that
    if failure {
      ret += 0b100;
    }
  }

  // Check if ChBe is being worked on
  if arr_labs[2] {
    // Variable that lets us know if there was a failure in executing a file operation
    let mut failure: bool = false;
    // Left half of template for ChBe
    let chbe_template_left: String = "\\\\e1-chbe-0".to_string();
    // Copy files to all 8 of the ChBe systems
    for i in 1..9 {
      let i_str: String = i.to_string();
      // Put everything together to get the final destination path
      let combined_chbe_dst_path = format!("{}{}{}{}", chbe_template_left, i_str, template_right, dst_path);
      // Perform file operation
      let rc = fs_helper(src_path, combined_chbe_dst_path.as_str());
      // If the file operation failed, update the failure variable
      if !rc {
        failure = true;
      }
    }
    // If any of the 8 file operations failed, change the return value to represent that
    if failure {
      ret += 0b1000;
    }
  }

  // Check if Segal is being worked on
  if arr_labs[3] {
    // Variable that lets us know if there was a failure in executing a file operation
    let mut failure: bool = false;
    // Left half of template for Segal
    let segal_template_left: String = "\\\\e1-segal-0".to_string();
    // Copy files to all 7 of the Segal systems
    for i in 1..8 {
      let i_str: String = i.to_string();
      // Put everything together to get the final destination path
      let combined_segal_dst_path = format!("{}{}{}{}", segal_template_left, i_str, template_right, dst_path);
      // Perform file operation
      let rc = fs_helper(src_path, combined_segal_dst_path.as_str());
      // If the file operation failed, update the failure variable
      if !rc {
        failure = true;
      }
    }
    // If any of the 7 file operations failed, change the return value to represent that
    if failure {
      ret += 0b10000;
    }
  }

  // Check if MCC is being worked on
  if arr_labs[4] {
    // Variable that lets us know if there was a failure in executing a file operation
    let mut failure: bool = false;
    // Left half of template for MCC
    let mcc_template_left: String = "\\\\e1-mcc-0".to_string();
    // Copy files to all 26 of the MCC systems
    for i in 1..27 {
      let i_str: String = i.to_string();
      // Put everything together to get the final destination path
      let combined_mcc_dst_path = format!("{}{}{}{}", mcc_template_left, i_str, template_right, dst_path);
      // Perform file operation
      let rc = fs_helper(src_path, combined_mcc_dst_path.as_str());
      // If the file operation failed, update the failure variable
      if !rc {
        failure = true;
      }
    }
    // If any of the 26 file operations failed, change the return value to represent that
    if failure {
      ret += 0b100000;
    }
  }

  // Return our encoded return value
  return ret;
}

// A handler must be generated for every custom command
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![handle_form_submit])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
