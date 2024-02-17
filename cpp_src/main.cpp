#include <iostream>
#include <fstream>
#include <string.h>


const int PASS_LEN = 6;
const int NAME_LEN = 6;
char * read_password(std::string file_name){
    std::ifstream file(file_name);
    std::string pass_out;
    std::getline(file,pass_out);
    return pass_out.data();
}

class Student {
    public:
        char name[PASS_LEN];
        char * getPass(){
            return pass; 
        }

    private:
        char * pass = read_password("password.txt");
        uint32_t ssn;
};


int main(void){
    std::cout << "Enter Name: \n";
    Student * s = new Student() ;

    std::cin >> s->name;
    std::cout << "Hello " << s->name << std::endl;
    std::cout << "Please Enter Password: ";
    char password_in[NAME_LEN];
    std::cin >> password_in;
    char * pass =s->getPass();
    if (0 == strcmp(password_in,pass)){
        std::cout << "Correct!\n";
    } 
    else{

    }
    std::cout << pass;
    std::cout << read_password("password.txt");
}