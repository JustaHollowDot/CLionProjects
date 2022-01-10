#include <iostream>
using namespace std;

int main() {
    string key, message, decrypted;
    int length;

    cin >> key >> length >> message;

    for (char i : message) {
        if (key.find(i) - 1 < -1) {
            decrypted.push_back(key.at(key.find(i) - 1));
        } else decrypted.push_back(key.back());
    }

    cout << decrypted;

    return 0;
}
