This is the README bundled with dtl.exe, a GUI designed to easily distribute files across lab machines managed by McCormick PC Support.

In general, the tool is fairly self-explanatory but this README will explain the aspects of it that are not immediately apparent.

Before beginning, it's worth noting that use of this tool requires domain-level access on the network.

-------------------------------------------------------------------------------------------------------------------------------------------
Homepage:

Note that the first three of the fields described below are mandatory while the last one is optional. This is indicated in the UI with
a * next to the title of fields that are mandatory.

Labs:
In this section, simply check the box of whatever lab(s) you'd like to copy files to. Once you've done so for a lab, you'll notice that
the "Select systems" select box has become larger and interactable. You may now select whichever specific systems within the lab you'd
like to target. If you'd simply like to do all systems in the lab, you may leave the selection empty (or you could select everything).

Local source path:
In this section, you select the path of the file/directory you'd like to copy over on your local machine. Note that you should only use
one of the two buttons in this section. The top button is for selecting individual files and the bottom button is for selecting an entire
directory. After you choose which button you'd like to use, the other button will disappear, assuming you actually made a selection when
the dialog popped up.

Remote destination path:
In this section, you select the path of where you would like for the file/directory you chose earlier to end up. Once the dialog has popped
up, a legal selection would look something like this: "\\bodeen-01.mcc.northwestern.edu\c$\Users\wsadmin\Downloads". It's important to note
here that even though it would seem like the operation would only copy your selected file/directory to Bodeen system 1, that is actually
not the case. Behind the scenes, the GUI will parse your input and figure out that you want your selected file/directory to end up at
"c$\Users\wsadmin\Downloads" on all the machines in the lab(s) you selected.

Settings:
The default behavior of the tool when copying files over when there's already a pre-existing file in the destination of the same name and
extension is simply to skip the pre-existing file and not copy the new one over. In other words, if you try to copy "test.txt" from src
into a dst with a pre-existing "test.txt", the operation won't happen and the "test.txt" that was already on dst will stay. It's worth
noting that this will go into the log as a successful operation as this is the expected behavior of the tool, not an error. The user may
elect to override this default behavior and overwrite pre-existing files of the same name by checking the box labeled "Overwrite existing
files of same name".
-------------------------------------------------------------------------------------------------------------------------------------------
Continuing on:

Now, you can press the "Continue" button. If you missed one of the fields, pressing the "Continue" button will tell you that you must
respond to all three fields and will turn the header of the field(s) that you missed red. After fixing the issue and retrying the button,
it should let you continue. After a slight delay (that gets longer the more systems you have selected), you'll be met with either a sucess
or failure screen. In either case, there will be a log of operations that were completed or failed to complete. After browsing through the
log or skipping it altogether, you can close the GUI whenever you'd like.

Potential failures:
There are many ways for the file operations to fail but here are some common failures to check against:
    - There's currently a bug where if a system is not connected to the network (e.g., powered off, Ethernet unplugged), file operations
      will fail for that system and all systems in the same lab after that. I'm currently working on patching this.
    - If there's a file already in the destination with the same name as the file you're copying over from your local machine, the
      operation will fail. It doesn't matter if the contents of the file are different, all that matters is the file name.
    - I'll continue to add more here as I encounter them.
-------------------------------------------------------------------------------------------------------------------------------------------
Contacting me:

Info: Alex Kang, Class of 2024, McCormick PC Support workstudy
Email: alexkang2024@u.northwestern.edu
