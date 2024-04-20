## Getting Started

This app requires Rust for the backend and Flutter for the frontend. If you do not have this tooling installed, here are the setup instructions (Linux only).

### Installing Rust

1. **Install Rust:**
   Rust can be installed using `rustup`, which is the official installer and version management tool for Rust.

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   Follow the on-screen instructions to complete the installation. Typically, it involves hitting `Enter` to proceed with the default installation.

2. **Configure your environment:**
   `rustup` typically configures your shell to include Cargo’s bin directory (where Rust’s build tools and dependencies are installed) in your `PATH`. If it doesn’t, you can manually add it by placing the following line in your `.bashrc` or `.bash_profile`:

   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

   Reload your bash configuration:

   ```bash
   source ~/.bashrc
   ```

3. **Verify the installation:**
   Check if Rust has been installed correctly by running:

   ```bash
   rustc --version
   ```

### Installing Flutter

1. **Install Flutter:**
   First, install the base dependencies needed for Flutter:

   ```bash
   sudo apt update
   sudo apt install -y git unzip xz-utils curl
   ```

2. **Download the Flutter SDK:**
   You can download the stable version of Flutter SDK and extract it. The following commands fetch the latest stable version and set it up in your home directory:

   ```bash
   cd ~
   curl -LO https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_3.0.5-stable.tar.xz
   tar -xvf flutter_linux_3.0.5-stable.tar.xz
   ```

3. **Set up the Flutter path:**
   Add Flutter to your `PATH` so that you can run Flutter commands in any terminal session:

   ```bash
   echo 'export PATH="$PATH:$HOME/flutter/bin"' >> ~/.bashrc
   source ~/.bashrc
   ```

4. **Verify the installation:**
   Once Flutter is installed, you can verify it by running:

   ```bash
   flutter doctor
   ```

   This command checks your environment and displays a report to the terminal window. It’s likely that `flutter doctor` will show some issues related to Android toolchain setup since you’re using WSL2 and might not have GUI support configured. For now, focus on Flutter being correctly set up to run and compile.

### Special Configurations for WSL2
When using WSL2, especially with GUI applications or mobile development environments like Android Studio, you might face issues related to GUI support and device connectivity (USB). Since you’re focusing on backend development with Rust and Flutter UI which can compile for web initially, these won't affect your immediate setup. For future needs, you might consider options like connecting WSL2 to a Windows-based Android emulator or configuring X11 forwarding for GUI apps.

This setup should get you started on your project development with Rust and Flutter on Ubuntu via WSL2. Let me know if you encounter any issues or need further assistance!