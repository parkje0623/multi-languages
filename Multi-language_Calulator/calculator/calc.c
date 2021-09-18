//swig -python -py3 calc.i
//gcc -fPIC -c calc.c calc_wrap.c -I/usr/include/python3.8
//ld -shared calc.o calc_wrap.o -o _calc.so

#include "calc.h"
#include <math.h>
#include <string.h>
#include <limits.h>
#include <stdbool.h>
#include <stdlib.h>

//Calcuate the CGPA
float gpa_calculate(float curr_gpa, float units, char *course1, char *course2, char *course3, float unit1, float unit2, float unit3) {
    float grade1 = letter_grade(course1);
    float grade2 = letter_grade(course2);
    float grade3 = letter_grade(course3);

    if (grade1 < 0) {
        unit1 = 0;
    } else if (grade2 < 0) {
        unit2 = 0;
    } else if (grade3 < 0) {
        unit3 = 0;
    }

    float sum_gpa = curr_gpa*units + grade1*unit1 + grade2*unit2 + grade3*unit3;
    float sum_units = units + unit1 + unit2 + unit3;

    float result = round((sum_gpa/sum_units)*100)/100;
    return result;
}

//Convert letter grade to a float value (to matching GPA value)
float letter_grade(char *course_grade) {
    float grade = 0.0;
    if (strcmp(course_grade, "a+") == 0) {
        grade = 4.33f;
    } else if (strcmp(course_grade, "a") == 0) {
        grade = 4.00f;
    } else if (strcmp(course_grade, "a-") == 0) {
        grade = 3.66f;
    } else if (strcmp(course_grade, "b+") == 0) {
        grade = 3.33f;
    } else if (strcmp(course_grade, "b") == 0) {
        grade = 3.00f;
    } else if (strcmp(course_grade, "b-") == 0) {
        grade = 2.67f;
    } else if (strcmp(course_grade, "c+") == 0) {
        grade = 2.33f;
    } else if (strcmp(course_grade, "c") == 0) {
        grade = 2.00f;
    } else if (strcmp(course_grade, "c-") == 0) {
        grade = 1.67f;
    } else if (strcmp(course_grade, "d") == 0) {
        grade = 1.00f;
    } else if (strcmp(course_grade, "f") == 0) {
        grade = 0.00f;
    } else {
        grade = -1.00f;
    }

    return grade;
}

//Calculate the target CGPA with remaning units, current CGPA and current units
float target_calculate(float curr_cgpa, float curr_units, float target_cgpa, float remain_units) {
    float curr_total_gpa = curr_cgpa * curr_units;
    float total_units = curr_units + remain_units;
    float target_total_gpa = target_cgpa * total_units;

    float needed_gpa = target_total_gpa - curr_total_gpa;
    float result = round((needed_gpa/remain_units)*100)/100;

    if (result > 4.33) {
        result = -1;
    }

    return result;
}

//export factorialC
unsigned long long factorialC(int n) {
    if (n == 0 || n == 1) {
        return 1;
    }

    unsigned long long result = 1;
    for (int i = 1;  i <= n; i++) {
        result *= i;
    }
    return result;
}

unsigned long long permutationC(int n, int r) {
    if (n == 0) {
        return 1;
    }

    unsigned long long result = factorialC(n)/factorialC(n-r);
    return result;
}

unsigned long long combinationC(int n, int r) {
    if (n == r) {
        return 1;
    }

    unsigned long long result = factorialC(r)*factorialC(n-r);
    result = factorialC(n)/result;
    return result;
}

bool isPrimeNum(unsigned int num) {
    if (num <= 1) {
        return false;
    }

    //Sieve of Eratosthenes
    unsigned int sieve = floor(sqrt(num));
    for (unsigned int i = 2; i <= sieve; i++) {
        if ((num%i) == 0) {
            return false;
        }
    }
    return true;
}

int primeNumberC(unsigned int num) {
    unsigned int count = 0;
    
    for (int i = 2; i <= num; i++) {
        if (isPrimeNum(i)) {
            count++;
        }
    }

    return count;
}
