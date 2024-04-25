## Development log

### 25 April 2024
My original intention was to use Flutter for frontend development and rust for the backend. But after wrestling with Android configuration for two days, I am abandoning Flutter
and will switch to Dioxsus for the full stack in rust. I may also consider spinning up the template app in RedwoodJS as well.

I have installed rust and tested the installation using my ProjectEulerRust. The project was using the Stopwatch crate which triggers a compiler error regarding reference `lifetimes`.
I removed the crate and changed the code to use the system timer instead.

