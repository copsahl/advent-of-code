#include <iostream>
#include <fstream>

//Answer = 3315383

using namespace std;

int requiredFuel(int modules);

int main(){

    // Fuel required to launch a given *module* is based on its *mass*.
    // Specifically, to find the fuel required for a module, take its mass, divide by 3, round down, and subtract 2. 

    int modules;
    int total = 0;
    int fuel;
    ifstream fp;
    int i;

    fp.open("one.txt");

    if(!fp.is_open()){
        cout << "File failed to open" << endl;
        return 1;
    }
    total = 0;
    i = 0;
    while(i < 100){
        fp >> modules;
        fuel = requiredFuel(modules);
        cout << "Fuel: " << fuel << "/" << modules << endl;
        total += fuel;
        i++;
    }

    cout << "Total: " << total << endl;
    fp.close();
    return 0;

}

int requiredFuel(int modules){

    int fuel;
    fuel = modules / 3;
    fuel -= 2;
    return fuel;
}
