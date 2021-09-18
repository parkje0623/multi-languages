# vagrant up
# vagrant ssh
# cd /vagrant
# cd cmpt383_prj1/calculator
# python3 main.py
# Go to localhost:5000
# https://prateekvjoshi.com/2016/03/08/how-to-create-a-web-server-in-python-using-flask/
# sudo apt install python3-pip -- if do not have pip
# sudo apt install swig -- if not already
# sudo apt install golang-go
# go build -o concurrent.so -buildmode=c-shared concurrent.go

from flask import Flask, render_template, request
import calc as c_calc
from ctypes import *
c_func = cdll.LoadLibrary("./_calc.so")
# Argument types for calculating target gpa
target_calc = c_func.target_calculate
target_calc.argtypes = [c_float, c_float, c_float, c_float]
target_calc.restype = c_float
# Result type for calculating CGPA
gpa_calc = c_func.gpa_calculate
gpa_calc.restype = c_float

go_func = cdll.LoadLibrary("./golanguage/concurrent.so")

PORT = 5000

app = Flask(__name__)

# Main Page
@app.route('/')
@app.route('/about')
def render():    
    return render_template('about.html')

# Switch page to CGPA Calculator page
@app.route('/cgpa')
def about():    
    return render_template('home.html', title='About')

# Switch page to Target CGPA Calculator page
@app.route('/target')
def target_gpa():    
    return render_template('target.html', title='Target CGPA')

# Connection to Factorial Calculator Page
@app.route('/factorial')
def factorial_page():
    return render_template('factorial.html')

# Connection to Prime NUmber Calculator Page
@app.route('/primeNum')
def prime_page():
    return render_template('primeNum.html')

# Connection to Fibonacci Page
@app.route('/fibonacci')
def fib_page():
    return render_template('fibonacci.html')

# Target CGPA Calculator (when received inputs -- method 'POST' from the users)
@app.route('/target_cgpa', methods=["POST"])
def target():    
    if request.method == 'POST':
        if request.form['submit_btn'] == 'Reset':
            return render_template('target.html', title='Target CGPA')

        cgpa = request.form['curr_gpa']
        units = request.form['total_unit']
        target_gpa = request.form['target_gpa']
        remain_unit = request.form['remain_unit']

        if cgpa == '0':
            cgpa = float(0)
        elif units == '0':
            units = float(0)
        elif target_gpa == '0':
            target_gpa = float(0)
        elif remain_unit == '0':
            remain_unit = float(0)
        elif not cgpa or not units or not target_gpa or not remain_unit:
            return render_template('target.html', error="***All fields must be entered***", title='Target CGPA')
        
        if float(cgpa) < 0 or float(cgpa) > 4.33:
            return render_template('target.html', error1="Please enter a number from 0 to 4.33", title='Target CGPA')
        elif float(units) < 0:
            return render_template('target.html', error2="Please enter a positive number", title='Target CGPA')
        elif float(target_gpa) < 0 or float(target_gpa) > 4.33:
            return render_template('target.html', error3="Please enter a number from 0 to 4.33", title='Target CGPA')
        elif float(remain_unit) < 0:
            return render_template('target.html', error4="Please enter a positive number", title='Target CGPA')

        # ADD CALCULATION FROM C HERE
        if request.form['submit_btn'] == 'Calculate':
            result_gpa = c_func.target_calculate(float(cgpa), float(units), float(target_gpa), float(remain_unit))
            result_gpa = round(result_gpa, 2)
            if result_gpa == -1:
                return render_template('target.html', target_gpa="Target CGPA is not achievable with remaining units", title='Target CGPA')
            else:
                return render_template('target.html', target_gpa=str(result_gpa), title='Target CGPA')
        else:
            return render_template('target.html', title='Target CGPA')

    return render_template('target.html', title='Target CGPA')

# C/GPA Calculator (when received inputs -- method 'POST' from the users)
@app.route('/calculate_cgpa', methods=['POST'])
def calculate():
    if request.method == 'POST':
        if request.form['submit_btn'] == 'Reset':
            return render_template('home.html')

        cgpa = request.form['curr_gpa']
        units = request.form['total_unit']
        course1 = request.form['course1']
        course2 = request.form['course2']
        course3 = request.form['course3']
        number1 = request.form['number1']
        number2 = request.form['number2']
        number3 = request.form['number3']
        
        if cgpa == '0':
            cgpa = float(0)
        elif units == '0':
            units = float(0)
        elif not cgpa or not units or not course1 or not course2 or not course3 or not number1 or not number2 or not number3:
            return render_template('home.html', error="***All fields must be entered***") 

        if float(cgpa) < 0 or float(cgpa) > 4.33:
            return render_template('home.html', error1="Please enter a number from 0 to 4.33")
        elif float(units) < 0:
            return render_template('home.html', error2="Please enter a (positive) number")

        if request.form['submit_btn'] == 'Calculate':
            new_cgpa = c_func.gpa_calculate(c_float(float(cgpa)), c_float(float(units)), str(course1), 
                                            str(course2), str(course3), c_float(float(number1)), 
                                            c_float(float(number2)), c_float(float(number3)))
            new_cgpa = round(new_cgpa, 2)
            return render_template('home.html', cgpa=str(new_cgpa))
        else:
            return render_template('home.html')

    else:
        return render_template('home.html')

# Factorial Calculator (when received inputs -- method 'POST' from the users)
@app.route('/factorial_calc', methods=['POST'])
def factorial():
    if request.method == 'POST':
        if request.form['submit_btn'] == 'Factorial':
            fact = request.form['fact']
            if not fact:
                return render_template('factorial.html', error1="Enter a number")

            result = c_func.factorialC(int(fact))
            return render_template('factorial.html', factorial=result)

        elif request.form['submit_btn'] == 'Permutation':
            perm_n = request.form['perm_n']
            perm_r = request.form['perm_r']
            if not perm_n or not perm_r:
                return render_template('factorial.html', error2="Enter a number")

            if int(perm_r) > int(perm_n):
                return render_template('factorial.html', error2="r must be greater than n")

            result = c_func.permutationC(int(perm_n), int(perm_r))
            return render_template('factorial.html', permutation=result)

        elif request.form['submit_btn'] == 'Combination':
            comb_n = request.form['comb_n']
            comb_r = request.form['comb_r']
            if not comb_n or not comb_r:
                return render_template('factorial.html', error3="Enter a number")

            if int(comb_r) > int(comb_n):
                return render_template('factorial.html', error3="r must be greater than n")

            result = c_func.combinationC(int(comb_n), int(comb_r))
            return render_template('factorial.html', combination=result)
        
        else:
            return render_template('factorial.html')
    
    else:  
        return render_template('factorial.html')

# Prim Number Calculator (when received inputs -- method 'POST' from the users)
@app.route('/primeNum_calc', methods=['POST'])
def matrix_calc():
    if request.method == 'POST':
        if request.form['submit_btn'] == 'Prime':
            prime = request.form['prime_num']
            if not prime:
                return render_template('primeNum.html', error1="Enter a number")

            yes_or_no = c_func.isPrimeNum(int(prime))
            result = ""
            if yes_or_no:
                result = "Number " + prime + " is a prime number"
            else:
                result = "Number " + prime + " is not a prime number"
            return render_template('primeNum.html', prime_number=result)

        elif request.form['submit_btn'] == 'Count':
            prime_count = request.form['prime_count']
            if not prime_count:
                return render_template('primeNum.html', error2="Enter a number")

            result = c_func.primeNumberC(int(prime_count))
            return render_template('primeNum.html', prime_Count=result)
        
        else:
            return render_template('primeNum.html')
    
    else:
        return render_template('primeNum.html')

# Fibonacci Calculator (when received inputs -- method 'POST' from the users)
@app.route('/fibonacci_calc', methods=['POST'])
def fibonacci_cal():
    if request.method == 'POST':
        if request.form['submit_btn'] == 'Fibonacci':
            fib = request.form['fib']
            if not fib:
                return render_template('fibonacci.html', error="Enter a correct number")
            elif int(fib) == 1 or int(fib) == 0:
                return render_template('fibonacci.html', Fib_value="0")

            result = go_func.fib_calc(int(fib))
            return render_template('fibonacci.html', Fib_value=result)
        
        else:
            return render_template('fibonacci.html')

    else:
        return render_template('fibonacci.html')

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=PORT)
