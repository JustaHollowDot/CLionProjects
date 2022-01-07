#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

int main() {
    long long int n, k, K0, K1, rest = 0, diff = 0;

    cin >> n >> k >> K1;

    K0 = K1;

    vector<int> deltakere(k - 1, 0);

    for (int i = 0; i < k - 1; i++) {
        cin >> deltakere[i];
    }

    sort(deltakere.begin(), deltakere.end());

    for (int a: deltakere) {
        cout << a << endl;
    }

    if (K1 >=deltakere.back()) {
        cout << 0;
        return 0;
    }

    for (int i = 2; K1 <= deltakere.back(); i++) {
        cout << "størrelse: "<< deltakere.size() << " K1: "  << K1 << endl;
        diff = (deltakere.back() - K1 + rest) / i;
        cout << deltakere.back() << endl;
        rest = (deltakere.back() - K1) % i;
        cout << "rest" << rest << endl;
        K1 += diff;

        deltakere.pop_back();
        if (deltakere.empty())
            break;
    }

    if (rest > 0)
        K1++;

    cout << K1 - K0 << endl;

    return 0;
}
