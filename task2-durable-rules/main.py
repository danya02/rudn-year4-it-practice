from durable.lang import *

with ruleset('computers'):
    @when_all(m.type == 'A')
    def computer_A(c):
        print('computers A')
        c.s.deadline_unset = True
        c.assert_fact('computers', {'test_id': 1})
        c.assert_fact('computers', {'test_id': 2})
        c.assert_fact('computers', {'test_id': 3})
    
    @when_all((m.type == 'B') & (m.role == 'DDNS'))
    def computer_B_DDNS(c):
        print('computers B DDNS')
        c.s.deadline_unset = True
        c.assert_fact('computers', {'test_id': 2})
        c.assert_fact('computers', {'test_id': 3})
    
    @when_all((m.type == 'B') & (m.role == 'DNS'))
    def computer_B_DNS(c):
        print('computers B DNS')
        c.s.deadline_unset = True
        c.assert_fact('computers', {'test_id': 4})
        c.assert_fact('computers', {'test_id': 5})
    
    @when_all((m.type == 'B') & (m.role == 'gateway'))
    def computer_B_gateway(c):
        print('computers B gateway')
        c.s.deadline_unset = True
        c.assert_fact('computers', {'test_id': 3})
        c.assert_fact('computers', {'test_id': 4})
    
    @when_all((m.type == 'B') & (m.role == 'router'))
    def computer_B_router(c):
        print('computers B router')
        c.s.deadline_unset = True
        c.assert_fact('computers', {'test_id': 1})
        c.assert_fact('computers', {'test_id': 3})
    
    @when_all(pri(1), m.test_id == 1, none(m.deadline > 0))
    def test_1(c):
        print('test 1')
        if c.s.deadline_unset:
            c.assert_fact('computers', {'deadline': 3})
            c.s.deadline_unset = False

    @when_all(pri(2), m.test_id == 2, none(m.deadline > 0))
    def test_2(c):
        print('test 2')
        if c.s.deadline_unset:
            c.assert_fact('computers', {'deadline': 7})
            c.s.deadline_unset = False

    @when_all(pri(3), m.test_id == 3, none(m.deadline > 0))
    def test_3(c):
        print('test 3')
        if c.s.deadline_unset:
            c.assert_fact('computers', {'deadline': 10})
            c.s.deadline_unset = False
    
    @when_all(pri(4), m.test_id == 4, none(m.deadline > 0))
    def test_4(c):
        print('test 4')
        if c.s.deadline_unset:
            c.assert_fact('computers', {'deadline': 12})
            c.s.deadline_unset = False
    
    @when_all(pri(5), m.test_id == 5, none(m.deadline > 0))
    def test_5(c):
        print('test 5')
        if c.s.deadline_unset:
            c.assert_fact('computers', {'deadline': 14})
            c.s.deadline_unset = False
    
    @when_all(c.deadline << m.deadline>0)
    def deadline(c):
        print('deadline', c.deadline)


type = input('Computer type (A or B): ')
role = input('Computer role (DDNS, DNS, gateway, router): ')

print(post('computers', {'type': type, 'role': role, 'created_at': int(time.time())}))
print(get_facts('computers'))
