#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

struct specificMeet {
    int person1, person2;
    int day;
};

bool customer_sorter(specificMeet const& lhs, specificMeet const& rhs) {
    return lhs.day < rhs.day;
}

int main() {
    int antInhabitants, meets, incubation, sickTime, maks = 0;
    int person1, person2, day;

    cin >> antInhabitants >> meets >> incubation >> sickTime;

    vector<int> people (antInhabitants, -1);
    people[0] = 0;

    vector<specificMeet> meetings;

    for (int i = 0; i < meets; ++i) {
        cin >> person1 >> person2 >> day;
        specificMeet temp = {person1, person2, day};
        meetings.push_back(temp);
    }

    sort(meetings.begin(), meetings.end(), &customer_sorter);


    if (meetings.back().day < incubation) {
        cout << 1;
        return 0;
    }


    /*
    for (int x : people) {
        cout << x << endl;
    }
    for(auto &x : meetings)
    {
        cout<<x.person1<<":"<< x.person2<<":"<<x.day<< endl;
    }*/


    for (int i = 0; i < meets; ++i) {
        if (meetings[i].day < incubation - 1)
            continue;

        if (people[meetings[i].person1] == -1 && people[meetings[i].person2] != -1) {
            if (people[meetings[i].person2] + incubation > meetings[i].day)
                continue;
            else if (people[meetings[i].person2] + incubation + sickTime <= meetings[i].day)
                continue;
            people[meetings[i].person1] = meetings[i].day;
        }
        else if (people[meetings[i].person2] == -1 && people[meetings[i].person1] != -1) {
            if (people[meetings[i].person1] + incubation > meetings[i].day)
                continue;
            else if (people[meetings[i].person1] + incubation + sickTime <= meetings[i].day)
                continue;
            people[meetings[i].person2] = meetings[i].day;
        }
    }

/*

    for (int x : people) {
        cout << x << endl;
    }*/

    vector<int> max (meetings.back().day + incubation + sickTime + 10, 0);
    for (int x : people) {
        if (x != -1) {
            for (int i = x + incubation; i < x + incubation + sickTime; ++i) {
                max[i]++;
            }
        }
    }

    /*
    cout << endl;

    for (int x : max) {
        cout << x << endl;
    } */

    // cout << endl;

    int maksverdi = *max_element(max.begin(), max.end());

    cout << maksverdi;
    return 0;
}