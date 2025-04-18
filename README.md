requirements:

- [install ndk](https://developer.android.com/studio/projects/install-ndk)
- install rust toolchains:
```bash
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
```
- change the run app task to use an APK from an app bundle

todo: 
- [ ] write a readme
- [x] rename to PonevLauncher
- [ ] get rid of all the stuff the compose template left that we don need
- [x] add lgpl v3 license
- [x] implement proper downloading
- [x] implement integrity checking
- [ ] implement launching (literally top 1 priority)
- [x] implement piston meta caching
