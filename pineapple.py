# because im lazy
# this is the rust code converted to python
# i dont trust my rust skills yet
print("https://github.com/JoeyShapiro/Pineapple")
print("This program will add up the grades for a weighted total")
print("It should be noted that it treats each assignment with its own weight")
print("For example, if one assignment is 40/50 and another is 25/30")
print("The one out of 50 will be worth more")
print("Also if you give an \"*\" it will find the \"best fit\" grade\n")

# START
print("eg. \"quizes tests ...\"")
gs = input("Enter groups separated by a space: ") # get all groups

# init
total = 0
groups = gs.split(" ") # split into array with space as the delimenator
groups_grades = {}

for group in groups:
    w = input(f"What is the weight (eg. 80) of {group}: ") # the weight

    grades_got = [] # list of grades the user got
    grades_out = [] # list of grades the grades are out of
    print("Enter grades separated by space")
    print("If you do not know put \"*\"")
    print("eg. final1: */100")
    grades = input(f"{group}>: ") # get all grades
    group_grades = grades.split(" ") # delimit grades by " " again
    for grade in group_grades:
        grades_got.append(grade.split("/")[0]) # get the top (->10/15)
        grades_out.append(grade.split("/")[1]) # get the bottom (10/15<-)

    dict_group = {"w": w, "grades_got": grades_got, "grades_out": grades_out} # store in dict
    groups_grades[group] = dict_group # put dict in dict at index of dict

check_desire = False
for d in groups_grades: # check if a star or unknown exists
    dgroup = groups_grades[d]
    if "*" in dgroup["grades_got"]:
        check_desire = True
        break # dont need to keep looking, i think...

if check_desire: # if is do stuff maybe with systems of equations
    desired = input("What grade do you want (eg. 90): ")
    f = int(desired) / 100
    d = 0

    # if 1 is unknown (final check)
    for di in groups_grades: # for each group
        dgroup = groups_grades[di] # name it
        if "*" in dgroup['grades_got']: # if the unknown is in here
            group_unknown = dgroup # name it
        else: # do basic arithmatic
            int_got = [int(s) for s in dgroup['grades_got']] # create int array
            int_out = [int(s) for s in dgroup['grades_out']] # create int array
            d += sum(int_got) / sum(int_out) * (int(dgroup['w']) / 100) # get the sum with weight
    
    c = sum([int(s) for s in group_unknown['grades_out']]) # get the sum of the bottom of unknown
    a = 0
    for grade in group_unknown['grades_got']: # do math on unknown portion
        if grade is not "*":
            a += int(grade) # add numerical grade to a

    b = int(group_unknown['w']) / 100
            
    star = (((f - d) * c) / b) - a # solve for x (the unknown)
    print(f"You need a {star} on the * to get an {desired}")
    if star < 0: # if x is a negative number
        print("You can not get a LOW enough score to get that grade")
    elif star > 100: # if x is over 100%
        print("You can not get a HIGH enough score to get that grade")

else: # if there is no star and you are solving for total (because youre boring)
    for dgroup in groups_grades:
        total += sum(dgroup[grades_got]) / sum(dgroup[grades_out]) * (dgroup[w] / 100) # maybe remove "/100"
    print("total:", total)