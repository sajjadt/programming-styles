g++ -std=c++11 modules/module.cpp -shared -fPIC -o libModule.so
g++ plugins.cpp  -ldl -std=c++11 -o plugins
