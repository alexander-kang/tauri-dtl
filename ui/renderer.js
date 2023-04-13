// renderer.js

// API imports
const open = window.__TAURI__.dialog.open // Opening dialogs
const invoke = window.__TAURI__.invoke // Calling custom commands

// Global variables
var srcPath // File path to copy from
var dstPath // File path to copy to

// Runs when the submit button is pressed
// Handles form submission
const submit = document.getElementById('submit')
submit.addEventListener('click', function() {
    // Stores what labs they're working on in an array of booleans
    let bodeen = document.getElementById('lab1').checked
    let mse = document.getElementById('lab2').checked
    let chbe = document.getElementById('lab3').checked
    let segal = document.getElementById('lab4').checked
    let mcc = document.getElementById('lab5').checked
    let labs = [bodeen, mse, chbe, segal, mcc]

    // Checking that the user gave input for all the fields
    let labsInputEmpty = true
    for (let i = 0; i < labs.length; ++i) {
        if (labs[i] == true) {
            labsInputEmpty = false
        }
    }
    if (labsInputEmpty || !srcPath || !dstPath) {
        document.getElementById('form-error-text').innerHTML = "You must respond to every field of this form!"
    }
    // If everything looks fine, call the custom command `handle_form_submit`
    else {
        document.getElementById('form-error-text').innerHTML = ""
        // Note: the argument names are in camelCase here because the documentation said they should be (see: https://tauri.app/v1/guides/features/command/)
        invoke('handle_form_submit', { arrLabs: labs, srcPath: srcPath, dstPath: dstPath }).then(function(ret) {
            console.log(ret)
            // Handling the return value
            // To see the details about the encoding of the return value, look at the code in main.rs
            if (!ret) {
                // Successful operations so change the HTML accordingly
                // Note that I chose not to simply load a different HTML page since I need to do something if there's a failure (see below for more info)
                document.getElementById("body").innerHTML =
                `<h1>File Distribution GUI</h1>
                <br><br><br><br><br><br><br><br><br><br>
                <p id="success">Success! You can close this window now.</p>`
            } else {
                // Some operation failed so change the HTML accordingly
                document.getElementById("body").innerHTML =
                `<span id="failure">
                    <h1>File Distribution GUI</h1>
                    <br><br><br><br><br><br><br><br><br><br>
                    <p>Something seems to have gone wrong.</p>
                    <p id="failure-replace"></p>
                    <p>Double check the source and destination paths you provided.</p>
                    <p>You can close this window now.</p>
                </span>`

                // I couldn't think of a way to get the failure string into the HTML if we loaded a different page so that's why I simply changed the HTML
                let failureString = "Specifically, file operations with the following labs seem to have failed:\n"

                // Now go through the return value and find out which lab(s) failed
                let failureStringLabs = ""

                // Testing
                if (ret & 1) {
                    failureStringLabs += "TestingFailure "
                }
                // Look at Bodeen
                if ((ret >> 1) & 1) {
                    failureStringLabs += "Bodeen "
                }
                // Look at MSE
                if ((ret >> 2) & 1) {
                    failureStringLabs += "MSE "
                }
                // Look at ChBe
                if ((ret >> 3) & 1) {
                    failureStringLabs += "ChBe "
                }
                // Look at Segal
                if ((ret >> 4) & 1) {
                    failureStringLabs += "Segal "
                }
                // Look at MCC
                if ((ret >> 5) & 1) {
                    failureStringLabs += "MCC"
                }

                failureString = failureString + failureStringLabs

                document.getElementById('failure-replace').innerHTML = failureString
            }
        })
    }
})

// Runs when the file browse button on the local source path is pressed
// Opens the file browser dialog and updates the corresponding global variable accordingly
const srcButtonFile = document.getElementById('src-button-file')
srcButtonFile.addEventListener('click', async function() {
    // File dialog
    let path = await open({
        directory: false,
        multiple: false,
        title: "Select Source File"
    })

    // Gets rid of the extra <br> separating the select file and select folder buttons since we're getting rid of the select folder button
    document.getElementById('src-file-box').innerHTML =
    `Select the file to be transferred:<br>
    <div class="button-box">
        <button type="button" id="src-button-file">Select</button>
        <p id="src-path-file"></p>
    </div>`
    // Shows the user's selected file path after the select file button
    document.getElementById('src-path-file').innerHTML = path
    // Gets rid of the select folder button
    document.getElementById('src-folder-box').innerHTML = ""
    // Update internal state
    srcPath = path
})

// Runs when the folder browse button on the local source path is pressed
// Opens the folder browser dialog and updates the corresponding global variable accordingly
const srcButtonFolder = document.getElementById('src-button-folder')
srcButtonFolder.addEventListener('click', async function() {
    // Folder dialog
    let path = await open({
        directory: true,
        multiple: false,
        recursive: true,
        title: "Select Source Folder"
    })

    // Shows the user's selected folder path after the select folder button
    document.getElementById('src-path-folder').innerHTML = path
    // Gets rid of the select file button
    document.getElementById('src-file-box').innerHTML = ""
    // Update internal state
    srcPath = path
})

// Runs when the browse button on the remote destination path is pressed
// Opens the folder browser dialog and updates the corresponding global variable accordingly
const dstButton = document.getElementById('dst-button')
dstButton.addEventListener('click', async function() {
    // Folder dialog
    let path = await open({
        directory: true,
        multiple: false,
        recursive: false,
        title: "Select Destination Folder"
    })

    // Parse the path so that it can be used for file operations
    path = path.substring(path.indexOf("$") - 1)
    // Shows the user's post-parsing selected folder path after the select destination button
    document.getElementById('dst-path').innerHTML = path
    // Update internal state
    dstPath = path
})
