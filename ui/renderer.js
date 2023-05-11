// renderer.js

const open = window.__TAURI__.dialog.open
const invoke = window.__TAURI__.invoke

var srcPath
var dstPath

// Helper function that converts given array into a bit encoding
function convToBin(arr) {
    let ret = 0b0
    for (let i = 0; i < arr.length; ++i) {
        if (arr[i]) {
            ret = ret | (1 << i)
        }
    }
    return ret
}

// Handle form submission
const submit = document.getElementById('submit')
submit.addEventListener('click', function() {
    // Handle lab & system selections
    let bodeenSelected = document.getElementById('lab1-checkbox').checked
    let bodeenSystems = convToBin([...document.getElementById('lab1-select').options].map(option => option.selected))
    let mseSelected = document.getElementById('lab2-checkbox').checked
    let mseSystems = convToBin([...document.getElementById('lab2-select').options].map(option => option.selected))
    let chbeSelected = document.getElementById('lab3-checkbox').checked
    let chbeSystems = convToBin([...document.getElementById('lab3-select').options].map(option => option.selected))
    let segalSelected = document.getElementById('lab4-checkbox').checked
    let segalSystems = convToBin([...document.getElementById('lab4-select').options].map(option => option.selected))
    let mccSelected = document.getElementById('lab5-checkbox').checked
    let mccSystems = convToBin([...document.getElementById('lab5-select').options].map(option => option.selected))
    let labsSelected = [bodeenSelected, mseSelected, chbeSelected, segalSelected, mccSelected]
    let labsSystems = [bodeenSystems, mseSystems, chbeSystems, segalSystems, mccSystems]

    // Validate user input
    let labsInputEmpty = true
    for (let i = 0; i < labsSelected.length; ++i) {
        if (labsSelected[i] == true) {
            labsInputEmpty = false
        }
    }
    if (labsInputEmpty || !srcPath || !dstPath) {
        document.getElementById('form-error-text').innerHTML = "You must respond to every field of this form!"
        if (labsInputEmpty) {
            document.getElementById('labs-legend').style.color = 'red'
        } else {
            document.getElementById('labs-legend').style.color = 'black'
        }
        if (!srcPath) {
            document.getElementById('src-legend').style.color = 'red'
        } else {
            document.getElementById('src-legend').style.color = 'black'
        }
        if (!dstPath) {
            document.getElementById('dst-legend').style.color = 'red'
        } else {
            document.getElementById('dst-legend').style.color = 'black'
        }
    // If all good to go, call custom command
    } else {
        document.getElementById('form-error-text').innerHTML = ""
        // Note: the argument names are in camelCase here because the documentation said they should be (see: https://tauri.app/v1/guides/features/command/)
        invoke('handle_form_submit', { arrLabsSelected: labsSelected, arrLabsSystems: labsSystems, srcPath: srcPath, dstPath: dstPath }).then(function(ret) {
            if (ret.result) {
                // Success
                document.getElementById("body").innerHTML =
                `<h1>File Distribution GUI</h1><br>
                <p>Success! You can close this window whenever you want.</p>
                <h3>Log</h3>
                <div id="success-box-holder">
                    <div id="success-box">
                        <p id="success-replace"></p>
                    </div>
                </div>`

                document.getElementById('success-replace').innerHTML = "Successful operations:<br>" + ret.succ_log
            } else {
                // Fail
                document.getElementById("body").innerHTML =
                `<span id="failure">
                    <h1>File Distribution GUI</h1><br>
                    <p>Whoops, something seems to have gone wrong.</p>
                    <p>You can close this window whenever you want.</p>
                    <h3>Log</h3>
                    <div id="failure-box-holder">
                        <div id="success-box">
                            <p id="success-replace"></p>
                        </div>
                        <div id="failure-box">
                            <p id="failure-replace"></p>
                        </div>
                    </div>
                </span>`

                document.getElementById('success-replace').innerHTML = "Successful operations:<br>" + ret.succ_log
                document.getElementById('failure-replace').innerHTML = "Failed operations:<br>" + ret.fail_log
            }
        })
    }
})

// Toggle select tag based on state of corresponding lab's checkbox
const bodeenCheckbox = document.getElementById('lab1-checkbox')
bodeenCheckbox.addEventListener('click', function() {
    const select = document.getElementById('lab1-select')
    if (bodeenCheckbox.checked) {
        select.disabled = false
        select.size = "4"
    } else {
        select.disabled = true
        select.size = "1"
    }
})
const mseCheckbox = document.getElementById('lab2-checkbox')
mseCheckbox.addEventListener('click', function() {
    const select = document.getElementById('lab2-select')
    if (mseCheckbox.checked) {
        select.disabled = false
        select.size = "4"
    } else {
        select.disabled = true
        select.size = "1"
    }
})
const chbeCheckbox = document.getElementById('lab3-checkbox')
chbeCheckbox.addEventListener('click', function() {
    const select = document.getElementById('lab3-select')
    if (chbeCheckbox.checked) {
        select.disabled = false
        select.size = "4"
    } else {
        select.disabled = true
        select.size = "1"
    }
})
const segalCheckbox = document.getElementById('lab4-checkbox')
segalCheckbox.addEventListener('click', function() {
    const select = document.getElementById('lab4-select')
    if (segalCheckbox.checked) {
        select.disabled = false
        select.size = "4"
    } else {
        select.disabled = true
        select.size = "1"
    }
})
const mccCheckbox = document.getElementById('lab5-checkbox')
mccCheckbox.addEventListener('click', function() {
    const select = document.getElementById('lab5-select')
    if (mccCheckbox.checked) {
        select.disabled = false
        select.size = "4"
    } else {
        select.disabled = true
        select.size = "1"
    }
})

// Handle file browsing for src path
const srcButtonFile = document.getElementById('src-button-file')
srcButtonFile.addEventListener('click', async function() {
    let path = await open({
        directory: false,
        multiple: false,
        title: "Select Source File"
    })

    // If path is null, the user cancelled the dialog
    if (path) {
        // Clean up UI
        document.getElementById('src-file-box').innerHTML =
        `Select the file to be transferred:<br>
        <div class="button-box">
            <button type="button" id="src-button-file">Select</button>
            <p id="src-path-file"></p>
        </div>`
        document.getElementById('src-path-file').innerHTML = path
        document.getElementById('src-folder-box').innerHTML = ""
        document.getElementById('warning-note').innerHTML = ""

        srcPath = path
    }
})

// Handle folder browsing for src path
const srcButtonFolder = document.getElementById('src-button-folder')
srcButtonFolder.addEventListener('click', async function() {
    let path = await open({
        directory: true,
        multiple: false,
        recursive: true,
        title: "Select Source Folder"
    })

    // If path is null, the user cancelled the dialog
    if (path) {
        // Clean up UI
        document.getElementById('src-path-folder').innerHTML = path
        document.getElementById('src-file-box').innerHTML = ""
        document.getElementById('warning-note').innerHTML = ""

        srcPath = path
    }
})

// Handle folder browsing for dst path
const dstButton = document.getElementById('dst-button')
dstButton.addEventListener('click', async function() {
    let path = await open({
        directory: true,
        multiple: false,
        recursive: false,
        title: "Select Destination Folder"
    })

    // If path is null, the user cancelled the dialog
    if (path) {
        // Parse the path so that it can be used for file operations
        path = path.substring(path.indexOf("$") - 1)

        // Clean up UI
        document.getElementById('dst-path').innerHTML = path

        dstPath = path
    }
})
