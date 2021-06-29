# Basic ESP32 Scaffold

The purpose of this project is to provide a stripped down basic scaffold from which other ESP32 projects can easily be built. It's built specifically to run in VSCode or CLion in windows with a custom-compiled rustc that supports the xtensa instruction set, so if you're in linux you'll probably want to use some other project. Probably the one I stole most of this code from.

I stole most of this code from https://github.com/MabezDev/xtensa-rust-quickstart, but it's licensed under apache/MIT, so they won't mind.

In order to make this work, we had to do the following:

- compile rust-xtensa fork to stage 2 from scratch per the instructions here: https://github.com/MabezDev/xtensa-rust-quickstart

- Once you've compiled the rust-xtensa fork to stage 2, you will also need to compile cargo (run this in the rust-xtensa root directory):
```shell
python.exe x.py build --stage 2 cargo
```

- After cargo is compiled, if you're planning to use CLion you'll need to copy the cargo.exe file from the  `C:\rust_code\rust-xtensa\build\x86_64-pc-windows-msvc\stage2-tools-bin` directory to the `C:\rust_code\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\` directory. If you recompile anything in stage 2 you'll probably need to repeat this step, as it deletes the files in that folder. The reason for this is that the toolchain in CLion expects both cargo and rustc to be in the same folder.

- If you intend to use this version of rust alongside the typical rust installation, you'll probably want to set up a permanent powershell alias for the rust-xtensa compiled cargo. To do this, you'll need to find out where powershell thinks your profile file is located. Run `$profile` in a powershell window, and note the path it gives you. Mine was `C:\Users\<username>\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`.

- Create the file that was shown in the $profile path, and add the new aliases to it. These are the only lines in my profile file, so the entire file looks like this:
```powershell
Set-Alias xtensa-cargo C:\rust_code\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\cargo.exe
Set-Alias xtensa-rustc C:\rust_code\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe
```

- The aliases will allow you to call the xtensa specific cargo and rustc commands without interfering with your normal rust installation. I've also made the flashing powershell script use these aliases, so you may need to modify that if you're just adding the xtensa-rustc to your path and not using a typical installation alongside it.

- Once you've got the xtensa-cargo alias set up, continue to follow the instructions from the 'Installing Tools' header in the https://github.com/MabezDev/xtensa-rust-quickstart link. Be sure to substitute `xtensa-cargo` for their normal `cargo` commands. You will need to install xtensa-esp32-elf (make sure to add this to the system path variable as well), cargo-xbuild, cargo-espflash, and esptool for everything to work correctly.

- Below are some editor-specific settings you will need to deal with to get VSCode and CLion to recognize the custom rust-xtensa files.

## VSCode Config:

- Set environment variables for this project folder in settings.json (adjust to match your actual paths as needed):
```json
"terminal.integrated.env.windows": {
    "RUSTC": "C:\\rust_code\\rust-xtensa\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe",
    "XARGO_RUST_SRC": "C:\\rust_code\\rust-xtensa\\library\\",
    "CUSTOM_RUSTC": "C:\\rust_code\\rust-xtensa\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe"
}
```
- Set the rust-analyzer settings to use xargo and the correct feature flags for the build (you will need to install xargo and adjust the path to match your actual paths as needed):
```json
"rust-analyzer.runnables.overrideCargo": "C:\\rust_code\\rust-xtensa\\build\\x86_64-pc-windows-msvc\\stage1-tools\\x86_64-pc-windows-msvc\\release\\.cargo\\bin\\xargo.exe",
"rust-analyzer.cargo.features": [
    "xtensa-lx-rt/lx6",
    "xtensa-lx/lx6",
    "esp32-hal"
]
```

## CLion Config:
- Set the environment variables for the terminal (Press Ctrl+Alt+S and add them in the `Tools > Terminal` menu)
```
RUSTC: C:\rust_code\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe
CUSTOM_RUSTC: C:\rust_code\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe
XARGO_RUST_SRC: C:\rust_code\rust-xtensa\library
CARGO: C:\rust_code\rust-xtensa\build\x86_64-pc-windows-msvc\stage2\bin\cargo.exe
```

## To Flash to the Chip:

- flash command :
```powershell
xtensa-cargo espflash --chip esp32 --speed 115200 --features="xtensa-lx-rt/lx6,xtensa-lx/lx6,esp32-hal" COM#
```
- Alternatively just run `./flash.ps1 COM#` in the root directory of this project.

- When running the flash command, to get the chip to talk, we had to connect to and then disconnect from the COM port in putty first.

- 