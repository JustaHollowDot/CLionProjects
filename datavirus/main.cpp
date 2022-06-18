#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

class UnionFind {
private :
    vector <int > p, rank , setSize ;
//p er lederen til hver enkelt node
// rank er kun for o p t i m a l i s e r i n g naar vi slaar sammen grupper
// setSize er antall noder i hver gruppe
    int numSets ; // antall grupper
public :
    UnionFind ( int N) {
        setSize . assign (N, 1) ; // er kun 1 i hver gruppe
        numSets = N; //N grupper
        rank . assign (N, 0) ;
        p. assign (N, 0) ;
        for ( int i = 0; i < N; i++)
            p[i] = i;
    }
    int findSet ( int i) {
        return (p[i] == i) ? i : (p[i] = findSet (p[i]) ) ;
//p[i]= findSet (p[i]) er kun for aa optimalisere ,
// slik at den husker hvem som er lederen til neste gang
    }
    bool isSameSet ( int i, int j) {
        return findSet (i) == findSet (j);
    }
    void unionSet ( int i, int j) {
        if (! isSameSet (i, j)) {
            numSets--;
            int x = findSet (i) , y = findSet (j);
// rank holder avstanden fra lederen til sine medlemmer kort
            if ( rank [x] > rank [y]) {
                p[y] = x;
                setSize [x] += setSize [y];
            } else {
                p[x] = y;
                setSize [y] += setSize [x];
                if ( rank [x] == rank [y])
                    rank [y ]++;
            }
        }
    }
    int numDisjointSets () { return numSets ; }
    int sizeOfSet ( int i) { return setSize [ findSet (i) ]; }
};

bool sortCol(const vector<int>& v1,
             const vector<int>& v2 ) {
    return v1.size() < v2.size();
}

int main() {
    int computers, connections;

    cin >> computers >> connections;

    vector < vector <int > > edge (computers);

    vector<int> a, b;
    int inputA, inputB;

    for (int i = 0; i < connections; ++i) {
        cin >> inputA >> inputB;
        a.push_back(inputA);
        b.push_back(inputB);

        edge[inputA].push_back (inputB);
        edge[inputB].push_back (inputA);
    }

    for (int i = 0; i < edge.size(); ++i) {
        edge[i].push_back(i);
    }

    sort(edge.begin(), edge.end(), sortCol);


    for (int i = 0; i < edge.size(); ++i) {
        cout << i << ": ";
        for (int x : edge[i]) {
            cout << x << " ";
        } cout << endl;
    } cout << endl << "minValue: ";

    vector < double > minValue (edge[computers - 1].size());
    for (int i = 2; i < edge[computers - 1].size(); ++i)
        minValue[i] = (computers / i * (computers / i - 1) / 2 * i);


    for (double x : minValue) {
        cout << x << " ";
    } cout << endl;

    int maxConnections = 0;
    int datamaskin;
    int temp = computers;
    vector <int> maxLeaders;

    int forbindelser = 0;
    int minForbindelser = 1000;

    while (temp--) {
        UnionFind test(computers);
        datamaskin = edge[temp].back();

        if (forbindelser < minValue[datamaskin] && temp != computers - 1) {
            //cout << "dette er en viktig hendelse!!" << endl;
            continue;
        }

        //cout << "rekkefølge datamaskiner: " << datamaskin << " ";

        for (int j = 0; j < connections; ++j) {
            if (datamaskin != a[j] && datamaskin != b[j]) {
                test.unionSet(a[j], b[j]);
            }
        }

        int sets = test.numDisjointSets();
        vector<int> leaders(1, 0);

        for (int i = 0; i < computers; ++i) {
            bool harLeder = false;
            if (i != datamaskin && i + 1 != datamaskin) {
                for (int j = 0; j < leaders.size(); ++j) {
                    if (test.isSameSet(leaders[j], i))
                        harLeder = true;
                }
                if (!harLeder) {
                    leaders.push_back(i);
                }
            }
            if (leaders.size() > maxLeaders.size())
                maxLeaders = leaders;
            if (leaders.size() == sets)
                break;
        }
        //cout << " ledere: " << leaders.size() << endl;

        forbindelser = 0;

        for (int maxLeader : maxLeaders) {
            forbindelser += (test.sizeOfSet(maxLeader)) * (test.sizeOfSet(maxLeader) - 1) / 2;
        }

        //cout << "forbindelser: " << forbindelser << endl;
        if (forbindelser < minForbindelser)
            minForbindelser = forbindelser;
    }

    /*
    cout << "maxLedere: "<< maxLeaders.size() << endl;
    for (int maxLeader : maxLeaders) {
        cout << maxLeader << " ";
    }
    cout << endl;*/

    /*cout << "\nmaks forbindelser: "<< maxConnections << endl;


    if (maxConnections == 3) {
        if (!((computers-1) % 2)) {
            cout << "dette er svaret (3, oddetall): " << ((computers/2) * (computers/2 - 1));
            return 0;
        } else {
            cout << "dette er svaret (3, partall): " << ((computers/2) * (computers/2 - 1)) / 2 + ((computers/2 - 1) * (computers/2 - 2)) / 2;
            return 0;
        }
    } else if (maxConnections == 2) {
        cout << "dette er svaret (2): " << ((computers-1) * (computers - 2)) / 2;
        return 0;
    } */


    cout << minForbindelser;


    return 0;
}

/*
UnionFind UF (4) ;
UF. unionSet (0 , 1) ;
UF. unionSet (2 , 3) ;
if (UF.isSameSet (0 , 3) )
cout << " Gates og Zuckerberg er venner \n";
else
cout << " Gates og Zuckerberg er ikke venner \n";
cout << " Antall grupper : " << UF. numDisjointSets () << endl ;
cout << " Antall personer i gruppen som Brin er med i: " << UF.
sizeOfSet (1) << endl ;
UF. unionSet (1 , 3) ;
cout << " Lederen til Zuckerberg er: " << UF. findSet (3) << endl ;*/
