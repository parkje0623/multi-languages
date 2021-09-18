#include <stdbool.h>

//Calcuate the CGPA
float gpa_calculate(float curr_gpa, float units, char *course1, char *course2, char *course3, float unit1, float unit2, float unit3);

//Convert letter grade to a float value (to matching GPA value)
float letter_grade(char *course_grade);

//Calculate the target CGPA with remaning units, current CGPA and current units
float target_calculate(float curr_cgpa, float curr_units, float target_cgpa, float remain_units);

//Calculate factorial
unsigned long long factorialC(int n);

//Calculate permutaition
unsigned long long permutationC(int n, int r);

//Calculate combination
unsigned long long combinationC(int n, int r);

//Helper function for finding whether a number is prime number or not
bool isPrimeNum(unsigned int num);

//Calculate All Prime NUmbers
int primeNumberC(unsigned int row);
