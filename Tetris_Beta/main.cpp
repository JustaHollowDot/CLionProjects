#include <iostream>
#include <vector>

using namespace std;

bool moveUp(vector <vector <bool>> tetris, int length, int temp, int start) {
    if (temp == 0) {
        return false;
    }
    for (int j = 0; j < length; ++j) {
        if (!tetris[temp - 1][start + j]){
        } else {
            return false;
        }
    }
    return true;
}

bool checkForRow(vector <vector <bool>> tetris, int width, int tempAntBlocks) {
    for (int i = 0; i < tempAntBlocks; ++i) {
        int temp = 0;
        for (int j = 0; j < width; ++j) {
            if (tetris[i][j])
                ++temp;
        }
        if (temp == width){
            return true;
        } else if (temp == 0)
            return false;
    }
    return false;
}

int deleteRow(vector <vector <bool>> tetris, int width, int tempAntBlocks) {
    for (int i = 0; i < tempAntBlocks; ++i) {
        int temp = 0;
        for (int j = 0; j < width; ++j) {
            if (tetris[i][j])
                ++temp;
        }
        if (temp == width){
            return i;
        }
    }
    return false;
}

int checkHeight(vector <vector <bool>> tetris, int width, int tempAntBlocks) {
    for (int i = 0; i < tempAntBlocks; ++i) {
        int temp = 0;
        for (int j = 0; j < width; ++j) {
            if (tetris[i][j])
                ++temp;
        } if (temp == 0)
            return i;
    }
    return -1;
}


int main() {
    int antBlocks, width, start, length, temp;

    bool check = false;

    cin >> antBlocks >> width;

    int tempAntBlocks = antBlocks;

    vector<vector<bool>> tetris(antBlocks, vector<bool> (width, false));

    for (int i = 0; i < antBlocks; ++i) {

        if (i >= tempAntBlocks) {
            if (check == true)
                break;
            i = tempAntBlocks - 1;
            check = true;
        }

        cin >> start >> length;
        for (int j = 0; j < length; ++j) {
            tetris[i][start + j] = true;
        }
        temp = i;
        while (moveUp(tetris, length, temp, start)) {
            for (int j = 0; j < length; ++j) {

                tetris[temp][start + j] = false;
                tetris[temp - 1][start + j] = true;
            }
            --temp;
        }

        if (checkForRow(tetris, width, tempAntBlocks)) {
            tetris.erase(tetris.begin() + deleteRow(tetris, width, tempAntBlocks));
            --tempAntBlocks;
        }

        if (checkHeight(tetris, width, tempAntBlocks) != -1) {
            cout << checkHeight(tetris, width, tempAntBlocks) << endl;
        }
    }

    return 0;
}
