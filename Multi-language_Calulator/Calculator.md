**CMPT 383 Final Project**  
**Overall Goal - Multiple Calculator Page**  
 - The main page lists out many different calculators. 
 - Each calculator page interacts with user input to serve its functionality 
 - Calculators include: CGPA/Target, Prime Number, Factorial, Fibonacci calculator
 - Goal of the project was to do web development using Python and communicate with other langauges that have faster computation speed to calculate mathematic computations. In this case, C and Go are used as fast computational languages.
 - Summary: Python for easy web development, C and Go for quickly solving intense computations.
  
**Three Languages Used:**  
 - C      -> Major Calculations of the project (CGPA/Target, Prime Number, Factorial calculator)
 - Python -> Web Development - Interaction with the users (Input/Output) and communicating with other languages
 - Go     -> Concurrently solving intense computation for faster computation of the program   
	     (Ex. Concurrently solving Fibonacci sequence)  

**Communication Methods Used:**  
 - SWIG   -> Use functions in C in Python  
 - ctypes -> Run Go (export) functions in Python (ctypes.cdll.LoadLibrary which loads exported go functions)
 - **Installation steps:**
 - SWIG: sudo apt install swig
 -		swig -python -py3 calc.i
 -		gcc -fPIC -c calc.c calc_wrap.c -I/usr/include/python3.8
 - 		ld -shared calc.o calc_wrap.o -o _calc.so
 - Golang: 
 -		cd golanguage
 -		go build -o concurrent.so -buildmode=c-shared concurrent.g
 -		cd ..
  
**Steps To Run The Project:**  
 1. 	vagrant up
 2. 	vagrant ssh
 3. 	python3 main.py
 4. 	Open a web browser and goto "localhost:5000" 
 - **If do not have Flask installed do:**
 - 		sudo apt install python3-pip
 - 		pip3 install flask

**Features To Look:**  
 The Web Developement (Python):
 - 	All buttons and clickable elements are functioning and action shown to the users
 -  Takes user inputs on the input area or propper error messages if wrong input used
 -  Outputs the correct result or proper error messages 
 -  Goal: Python for web development and communicating with C and Go to receive results of the calculations  
 
 Calculators (C and Go):
 -  Fast calculations for every calculators provided
 -  Goal: Much faster calculation speed provided compared to calculation using Python



