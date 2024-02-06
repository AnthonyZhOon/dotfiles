Notation:
- Identify entity as bullet points (with some detail about Key attr.)
- Indent under each entity for description of non-key attr.
- *Italics* for descriptions of relationships

## Analysis
- Fitness Centre (Each fitness centre is identified by a centre id)
	- The data which is to be recorded about a centre is the centre’s name, address and phone number. Each centre employs a number of staff to help it carry out its various activities.
	- *One of the appointed staff members of the centre, manages the centre (a center is required to have a manager assigned before it can operate).*
	- *As part of its business activities, each centre runs paid formal classes which members cansign up for.*
- Staff (Each staff member is identified by a staff id)
	- The	staff member’s first (or given) name, last (or family) name, mobile number, the date they started at the centre and role (the type of job they carry out) are recorded.
- Facility (The facility room numbers are unique within each centre.)
	- Each centre has a number of facilities such as Gymnasium, Spin Room, Swimming Pool,Aerobic Exercise Room, etc.
	- Each of these facilities is identified by a facility room number. These room numbers are not unique across Vitality Fitness Club, room numbers are reused within each centre - for example every centre will have a facility room numbered 1.
	- For a facility we wish to record the facility name, a description of the facility and its capacity (the number of participants it can accommodate).
	- *Each centre has at least one facility.*
- Class (A class, within a centre, is identified by a unique numeric centre assigned class number (these class numbers are not unique across Vitality Fitness Club - for example every centre will have a class number, say 10 ).)
	- The class description (eg. HIT), the start date and time, and the maximum number of participants are recorded.
	-  The number of sessions for each class and the class cost are also recorded.
	- Only the start date and time of the first session in each class is recorded.
	- *Some classes only run one session, others run several sessions in which each session will be run on the same day and time each week.*
	- *Each class makes use of a particular facility within the centre. If the class repeats, it always uses the same facility.*
	- *Classes are run by qualified staff members of the centre. A given class may be run by a single staff member or by several staff members.*
- Club Member (Vitality Fitness Club identifies the member with a unique member number, these member numbers are unique across Vitality Fitness Club.)
	- They also wish to record the member's name, their address, their phone number and the date which they joined Vitality Fitness Club.
	- Vitality Fitness Club is interested in recording whether a new member was referred (recommended to join) by a current member.
	- Vitality Fitness Club members are required to nominate a particular centre as their home centre.
	- Although they have a home centre, members are entitled to enrol for the classes run at any of the Vitality Fitness Club centres.
- Enrolment (Vitality Fitness Club wishes to record which members sign up for which classes.) *Many-to-Many between Class and Member, this is an associative entity that must exist to store data on this.*
	- Members who have enrolled in a particular class are required to pay for the class before the class actually begins, the date the member paid is recorded.
	- Payment is a once off payment for the class, covering all  sessions. If a class has more than one session and a member does not attend some sessions, the cost of the class does not change.
- Assessment (Qualified staff at each centre offer fitness assessments to members)
	- *these are optional, a member may not complete any assessments or they may complete several.*
	- The date the assessment was completed and the staff member who ran the assessment are recorded (only one assessment per member can be completed on any one day).
	- The fitness assessment records the members weight, blood pressure, BMI and VO2Max.


