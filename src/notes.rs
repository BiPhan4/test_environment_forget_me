//constants always immutable 
//convention is to for constants to use all UpperCase with _ 
/*
compiler able to evaluate limited set of operations at compile time which lets us choose to write out this value in a way
thats easier to understand and verify, rather than setting thsi constant to
the value 10800 

You can declare a new var with same name as prev var
first var is shadowed by the second
which means the second variable's value is what the program sees
when the var is used
We shadow a var by using the same vr name and repeating the use
of the let keyword as follows

shadow is not same as marking as mut 
becasue we get error if we try to reassign to this var without using let keyword
by using let we can perform some transforms on a val but have the var be immutable
after those transforms have been done 
also, another dif btwn mut and shadow is bc we're creating a new var when we use the ley 
keyword again
we can change the type of the value but reuse the same name
for ex, say our program asks a user to show how many spaces they want between
some tet by inputting space chars
thenwe want to store that inputas a number 


rust data aggregates such as structs, enums and tuples are dumb
You can define methods on them and make the data itself private
all they are unrelated types
no subtyping and no inheritance of data - aside from the case of 
Deref coercions

relationships are established using traits
how does trait operate? Traits is the web of meaning that glues all
the data types together
traits - no one to one correspondence between them and concepts 
from mainstream languages
thinking dynamically or statically?
in the dynamic case, traits are like Java or Go interfaces


*/