# time_in_words.rs

Explanation of the Code

1.	The time_in_words Function:
	•	This function accepts two parameters:
	        h: The current hour of the day.
	        m: The current minutes past the hour.
            
	•	A Vec<String> called numbers is created to store the word representation of numbers (e.g., “one”, “two”, “quarter”, etc.).

	•	The match statement handles different cases of m:
	        Case 0: If m == 0, return <hour> o' clock (e.g., “five o’ clock”).
	        Case 1: If m == 1, return one minute past <hour> (e.g., “one minute past five”).
	        Case 15, 30, and 45: Handle special cases like “quarter past”, “half past”, and “quarter to”.
	        Case 1..=29: If the minutes are less than or equal to 29, return <minutes> past <hour>.
	        Case 31..=59: If the minutes are greater than 30, calculate the remaining time to the next hour using 60 - m, and return <minutes> to <next hour>.
            
	•	The % 12 operation ensures the time stays within the 12-hour clock range.

2.	The main Function:
	•	Reads h (hour) and m (minutes) from the input.
	•	Calls the time_in_words function to compute the result.
	•	Writes the output to a file specified by the environment variable OUTPUT_PATH.