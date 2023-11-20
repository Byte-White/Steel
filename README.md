# Steel (still in development)
![steellang](https://github.com/Byte-White/Steel/assets/51212450/f5f5b626-bb68-455c-91d8-a3ea77eb77b2)


A code generator/compiler for GUI applications powered by Appazoid 


## Requirements
You should have Rust installed.


## How to use


`cargo run <file>`

`cargo run file.st`
file.st:
```cpp
window("Hello");

loginfo("Hello World");

button("mybutton");
exit 0;
```

this will generate a file named `az_compiled.h`