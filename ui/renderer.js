// renderer.js

// API imports
import { listen } from '@tauri-apps/api/event' // Listening to events from backend
import { open } from '@tauri-apps/api/dialog' // Opening dialogs
import { invoke } from '@tauri-apps/api/tauri' // Calling custom commands

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
    // Note that we didn't make it async because we're not really concerned about the speed of the operations here (there's not that many lab machines) and it being async could lead to some race conditions
    else {
        document.getElementById('form-error-text').innerHTML = ""
        // Note: the argument names are in camelCase here because the documentation said they should be (see: https://tauri.app/v1/guides/features/command/)
        invoke('handle_form_submit', { arrLabs: labs, srcPath: srcPath, dstPath: dstPath })
    }
})

// Runs when the file browse button on the local source path is pressed
// Opens the file browser by sending a command to main over IPC
const srcButtonFile = document.getElementById('src-button-file')
srcButtonFile.addEventListener('click', function(event) {
    ipcRenderer.send('srcOpenDialogFile')
})
ipcRenderer.on('srcSelectedFile', function(event, path) {
    document.getElementById('src-file-box').innerHTML =
    `<div id="src-file-box">
        Select the file to be transferred:<br>
        <div class="button-box">
            <button type="button" id="src-button-file">Select</button>
            <p id="src-path-file"></p>
        </div>
    </div>`
    document.getElementById('src-path-file').innerHTML = path
    document.getElementById('src-folder-box').innerHTML = ""
    srcPath = path
})

// Runs when the folder browse button on the local source path is pressed
// Opens the file browser by sending a command to main over IPC
const srcButtonFolder = document.getElementById('src-button-folder')
srcButtonFolder.addEventListener('click', function(event) {
    ipcRenderer.send('srcOpenDialogFolder')
})
ipcRenderer.on('srcSelectedFolder', function(event, path) {
    document.getElementById('src-path-folder').innerHTML = path
    document.getElementById('src-file-box').innerHTML = ""
    srcPath = path
})

// Runs when the browse button on the remote destination path is pressed
// Opens the file browser by sending a command to main over IPC
const dstButton = document.getElementById('dst-button')
dstButton.addEventListener('click', function(event) {
    ipcRenderer.send('dstOpenDialog')
})
ipcRenderer.on('dstSelected', function(event, path) {
    document.getElementById('dst-path').innerHTML = path[0].substring(path[0].indexOf("$") - 1)
    dstPath = path
})
