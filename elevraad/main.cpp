#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

int main() {
    long long int stemmer, kandidater, mineStemmer, rest = 0, diff = 0;

    cin >> stemmer >> kandidater >> mineStemmer;

    const int constMineStemmer = mineStemmer;

    vector<int> deltakereStemmer(kandidater - 1, 0);

    for (int i = 0; i < kandidater - 1; i++) {
        cin >> deltakereStemmer[i];
    }

    sort(deltakereStemmer.begin(), deltakereStemmer.end());

    if (mineStemmer >= deltakereStemmer.back()) {
        cout << 0;
        return 0;
    }

    for (int i = 2; mineStemmer <= deltakereStemmer.back(); i++) {
        // cout << "størrelse: " << deltakereStemmer.size() << " mineStemmer: " << mineStemmer << endl;
        diff = (deltakereStemmer.back() - mineStemmer + rest) / i;
        // cout << deltakereStemmer.back() << endl;
        rest = (deltakereStemmer.back() - mineStemmer) % i;
        // cout << "rest" << rest << endl;
        mineStemmer += diff;

        deltakereStemmer.pop_back();
        if (deltakereStemmer.empty())
            break;
    }

    if (constMineStemmer > 0)
        mineStemmer++;

    cout << stjalet << endl;

    return 0;
}
