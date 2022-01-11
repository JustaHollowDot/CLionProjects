#include <iostream>
#include <vector>
using namespace std;

struct person {
    int timeSick, immune;
    bool isImmune;
};

int main() {
    int antInhabitants, meets, incubation, sickTime, maks = 0;
    int person1, person2, day;

    cin >> antInhabitants >> meets >> incubation >> sickTime;

    vector<person> people (antInhabitants, {-1, -1, false});

    people[0].timeSick = incubation;
    people[0].immune = incubation + sickTime;

    for (int i = 0; i < meets; ++i) {
        cin >> person1 >> person2 >> day;

        if (people[person1].immune < day)
            people[person1].isImmune = true;
            //break?
        else if (people[person2].immune < day)
            people[person2].isImmune = true;
            //break?

        if (!people[person1].isImmune && !people[person2].isImmune) {
            if (people[person1].timeSick <= day && day < people[person1].immune && people[person2].timeSick == -1) {
                people[person2].timeSick = day + incubation;
                people[person2].immune = incubation + sickTime;
            }

            if (people[person2].timeSick <= day && day < people[person2].immune && people[person1].timeSick == -1) {
                people[person1].timeSick = day + incubation;
                people[person1].immune = incubation + sickTime;
            }
        }
    }
    for(auto &x : people)
    {
        cout<<x.timeSick<<":"<<x.immune<<endl;
    }

    int counter = 0;
    for (int i = 0; i < day; ++i) {
        counter = 0;

        for (int j = 0; j < antInhabitants; ++j) {
            if (people[j].timeSick <= i && i < people[j].immune) {
                    ++counter;
                    cout << "counter: " << counter << endl;
            }
            if (counter > maks)
                maks = counter;
        }
    }


    /*
    vector< pair<int,int> > persons (antInhabitants, make_pair(-1,-1));
    persons[0].first = 0 + incubation;
    persons[0].second = 0 + incubation + sickTime;

    for (int i = 0; i < meets; ++i) {
        cin >> person1 >> person2 >> day;

        if (persons[person1].first <= day && day < persons[person1].second && persons[person1].first != -1 && persons[person2].first == -1) {
            if (persons[person2].first == -1) {
                persons[person2].first = day + incubation;
                persons[person2].second = day + incubation + sickTime;
            }
        } else if (persons[person2].first <= day && day < persons[person2].second && persons[person2].first != -1 && persons[person1].first == -1) {
            if (persons[person1].first == -1) {
                persons[person1].first = day + incubation;
                persons[person1].second = day + incubation + sickTime;
            }
        }

    }

    for(auto &x : persons)
    {
        cout<<x.first<<":"<<x.second<<std::endl;
    }

    for (int i = 0; i < day; ++i) {
        int counter = 0;
        for (int j = 0; j < antInhabitants; ++j) {
            if (persons[j].first <= i && i < persons[j].second && persons[j].first != -1) {
                ++counter;
                cout << counter << " person: " << j << " ";
            }
        }
        cout << endl;

        if (counter > maks)
            maks = counter;
    }*/

    cout << maks;
    return 0;
}