#include <iostream>

struct Person {
    char name[6];
    char ssn[6];
};

class Student {
    public:
        char name[6];
        char pass[6];
        char * getPass(){
            return pass; 
        }

    private:
        uint32_t ssn;
};
int main(void){
    std::cout << "Enter Name: \n";
    Student * s = new Student() ;
    Person p ;
    std::cin >> p.name ;//s->name;
    std::cout << "Hello " << p.name << std::endl;
    // std::cout << "Pass: " << s->getPass() << std::endl;
    std::cout << "SSN: " << p.ssn << std::endl;
}