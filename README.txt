This is the README bundled with dtl.exe, a GUI designed to easily distribute files across lab machines managed by McCormick PC Support.

In general, the GUI is fairly self-explanatory but this README will explain the aspects of it that are not immediately apparent.

-------------------------------------------------------------------------------------------------------------------------------------------
Homepage:

Note that all three of the fields described below are mandatory.

Labs:
In this section, simply check the box of whatever lab(s) you'd like to copy files to.

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
-------------------------------------------------------------------------------------------------------------------------------------------
Continuing on:

Now, you can press the "Continue" button. If you missed one of the fields, pressing the "Continue" button will tell you that you must
respond to all three fields and will turn the header of the field(s) that you missed red. After fixing the issue and retrying the button,
it should let you continue. After all of the above, you'll be met with either a sucess or failure screen. In either case, there will
be a log of operations that were completed or failed to complete. After browsing through the log or skipping it altogether, you can close
the GUI whenever you'd like.

Potential failures:
There are many ways for the file operations to fail but here are some common failures to check against:
    - There's currently a bug where if a system is not connected to the network (e.g., powered off, Ethernet unplugged), file operations
      will fail for that system and all systems after that. I'm currently working on patching this.
    - If there's a file already in the destination with the same name as the file you're copying over from your local machine, the
      operation will fail. It doesn't matter if the contents of the file are different, all that matters is the file name.
    - I'll continue to add more here as I encounter them.
-------------------------------------------------------------------------------------------------------------------------------------------
Contacting me:

Info: Alex Kang, Class of 2024, McCormick PC Support workstudy
Email: alexkang2024@u.northwestern.edu
