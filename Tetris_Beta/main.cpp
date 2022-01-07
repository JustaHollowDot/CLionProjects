#include <iostream>
#include <vector>
using namespace std;

int main() {
    int antBlocks, width, start, length;

    cin >> antBlocks >> width;

    vector<vector<bool>> tetris(antBlocks, vector<bool> (width, false));

    for (vector<bool> tetris1D : tetris)
    {
        for (bool x : tetris1D)
        {
            cout << x << " ";
        }
        cout << endl;
    }
    cout << endl;

    for (int i = 0; i < antBlocks; ++i) {
        cin >> start >> length;
        for (int j = 0; j < length; ++j) {
            tetris[i][start + j] = true;

            for (int k = 0; k < width; ++k) {
                for (int l = 0; l < length; ++l) {

                }
            }
        }
        for (vector<bool> tetris1D : tetris)
        {
            for (bool x : tetris1D)
            {
                cout << x << " ";
            }
            cout << endl;
        }
        cout << endl;
    }

    return 0;
}
