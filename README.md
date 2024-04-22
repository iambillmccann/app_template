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

## Setting up Android Tooling for WSL 2

Since you're working on WSL2 and given the known issues with Android emulation performance in that environment, the optimal approach is to install Android Studio on your Windows system rather than directly on WSL2. This setup allows you to utilize the full performance capabilities of Windows for Android emulation and UI design while still developing your Flutter and Rust code in the Linux environment provided by WSL2. Here’s how you can set it up:

### Step 1: Install Android Studio on Windows

1. **Download Android Studio:**
   - Go to the [official Android Studio download page](https://developer.android.com/studio) and download the installer for Windows.

2. **Run the Installer:**
   - Execute the downloaded installer and follow the prompts to install Android Studio. During the installation, ensure you select the components you need, including the Android SDK, Android Virtual Device (AVD), and if needed, any specific APIs or tools.

3. **Configure Android Studio:**
   - Launch Android Studio after installation. It will guide you through a setup wizard, which includes downloading any additional SDK components required.

### Step 2: Configure WSL2 to Use the Android SDK from Windows

After installing Android Studio and the Android SDK on Windows, you need to configure your WSL2 environment to use the SDK from Windows. This involves setting environment variables in WSL2 that point to the SDK location on your Windows system.

1. **Locate the Android SDK on Windows:**
   - By default, the Android SDK is installed in `C:\Users\<YourUsername>\AppData\Local\Android\Sdk`. You can confirm this path in Android Studio under **Settings** > **Appearance & Behavior** > **System Settings** > **Android SDK**.

2. **Mount Windows Drives on WSL2:**
   - Windows drives are typically mounted under `/mnt/` in WSL2. For example, your `C:` drive is mounted at `/mnt/c`.

3. **Set Environment Variables in WSL2:**
   - Open your WSL2 terminal and add the following lines to your `.bashrc` or `.bash_profile` to set the SDK path:
     ```bash
     export ANDROID_HOME=/mnt/c/Users/<YourUsername>/AppData/Local/Android/Sdk
     export PATH="$PATH:$ANDROID_HOME/tools:$ANDROID_HOME/platform-tools"
     ```
   - Replace `<YourUsername>` with your actual Windows username.
   - Apply the changes by sourcing the profile:
     ```bash
     source ~/.bashrc
     ```

4. **Verify the Configuration:**
   - In your WSL2 terminal, check that the `adb` command (part of the platform-tools) is working:
     ```bash
     adb version
     ```

### Step 3: Integrating with Flutter

Ensure that Flutter in WSL2 uses the correct Android SDK path:

- Run:
  ```bash
  flutter config --android-sdk "/mnt/c/Users/<YourUsername>/AppData/Local/Android/Sdk"
  ```

### Step 4: Handling Emulators

Since you cannot run Android emulators efficiently in WSL2, use the emulators managed through Android Studio on Windows. When you need to test your app:
- Start the emulator via Android Studio on Windows.
- Use the Flutter tools in WSL2 to deploy to the emulator by targeting it as a connected device.

This setup leverages the strengths of both environments: Windows for Android-specific tooling and UI, and WSL2 for Linux-based development in Flutter and Rust. If you encounter any issues with this setup or need further assistance, feel free to ask!

