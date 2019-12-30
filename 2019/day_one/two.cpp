#include <iostream>
#include <fstream>

//Answer = 4970206

using namespace std;

int requiredFuel(int modules, int total);

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
    
    while(i < 100){
        fp >> modules;
        fuel = requiredFuel(modules, 0);
        cout << "Fuel: " << fuel << "/" << modules << endl;
        total += fuel;
        i++;
    }

    cout << "Total: " << total << endl;
    fp.close();
    return 0;

}

int requiredFuel(int modules, int total){

    int fuel;
    fuel = modules / 3;
    fuel -= 2;

    if(fuel <= 0){
        return total;
    }

    total += fuel;

    requiredFuel(fuel, total);

}
