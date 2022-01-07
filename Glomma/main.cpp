#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;

int main() {
    long long int N, K, Xg, temp, lengde;
    cin >> N >> K >> Xg;

    vector<int> address(N, 0);

    //lagrer alle adressene til husene
    for (int i = 0; i < N; ++i) {
        cin >> temp;
        address[i] = temp;
    }

    sort(address.begin(), address.end());

    //rask kalkulasjon hvis antall pakker og antall hus er likt
    if (N <= K) {

        if (Xg < address.back()) lengde += address.back() - Xg;
        if (Xg > address.front()) lengde += Xg - address.front();
        cout << lengde * 2 << endl;
        return 0;
    }

    //finner indeksen til lageret
    vector<int>::iterator it;
    it = lower_bound(address.begin(), address.end(), Xg);
    int idx;
    idx = it - address.begin();

    vector<int> mindreAddress(idx, 0);

    for (int i = 0; i < idx; ++i) {
        mindreAddress[i] = address[0];
        address.erase(address.begin());
    }

    lengde = 0;

    while (!address.empty()) {

        if (K > address.size()){
            lengde += address.back() - Xg;
            break;
        }
        lengde += address.back() - Xg;
        address.resize(address.size() - K);
    }

    int offset = 0;
    while (!mindreAddress.empty()) {
        if (K >= mindreAddress.size()) {
            lengde += Xg - mindreAddress.front();
            break;
        }
        lengde += Xg - mindreAddress.front();
        mindreAddress.erase(mindreAddress.begin(), mindreAddress.begin() + K);
        offset += K;
    }
    cout << lengde * 2 << endl;
    return 0;
}
