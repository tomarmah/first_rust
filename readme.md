/\*\*

- This module handles the flow of the ATM service.
- It allows:
-
- 1.  Bank owner to:
- - create a bank
- - add/remove CFO
- - see bank balance
- - see total bank deposits
- - see total bank withdrawals
-
- 2.  Banks CFO to:
- - Setup ATMs
- - See balances of all ATMs
- - See balance of a particular ATM
- - See notifications of low balance ATMs
- - Add/Remove Money loader
- - Send ATM load requests to money loaders
-
- 3.  Money Loaders to:
- - See all load requests
- - Accept/Reject load requests
- - Load money to ATMs
- - Request money to load ATM
- - See their balance
-
- 4.  Customers to:
- - Withdraw money from ATM
- - Deposit money into account
- - See their account balance
- - See their transaction history
- - Transfer money to another account
- - Change their PIN
    \*/

/\*\*

- A command consists of a label, description and options.
- The options of a command are the various kinds of actions it can perform.
- For example, the "role" command can have options like "switch", "list", "select"
- - switch: switch to a different role
- - list: list all available roles
- - select: select a role to perform actions
-
- -- bank commands -- {create, delete, add/remove cfo, view bank balance, view total deposits, view total withdrawals}
- -- to perform these commands user must be a bank owner
- -- atm commands -- {setup atm, view all atm balances, view atm balance, view low balance notifications, add/remove money loader, send load request}
- -- to perform these commands user must be a cfo
- -- load commands -- {view load requests, accept/reject load request, load money to atm, request money to load atm, view loader balance}
- -- to perform these commands user must be a money loader
- -- customer commands -- {withdraw money, deposit money, view account balance, view transaction history, transfer money, change pin}
- -- to perform these commands user must be a customer
- -- exit command -- {exit}
  \*/
