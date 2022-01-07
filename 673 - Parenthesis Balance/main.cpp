#include <iostream>
#include <string>
using namespace std;

int main() {
    int n;
    string parenthesis;


    cin >> n;
    cin.ignore();

    while (getline(cin, parenthesis)) {
        for (int i = 0; i < parenthesis.length(); ++i) {
            if (parenthesis[i] == ')') {
                for (int j = i; j >= 0; --j) {
                    if (parenthesis[j] == '(') {
                        parenthesis.erase(j,1);
                        parenthesis.erase(i-1,1);
                        i -= 2;
                        break;
                    }
                }
            } else if (parenthesis[i] == ']') {
                for (int j = i; j >= 0; --j) {
                    if (parenthesis[j] == '[') {
                        parenthesis.erase(j,1);
                        parenthesis.erase(i-1,1);
                        i -= 2;
                        break;
                    }
                }
            }
        }
        if (!parenthesis.length()) {
            cout << "Yes" << endl;
        } else {
            cout << "No" << endl;
        }
    }
}
