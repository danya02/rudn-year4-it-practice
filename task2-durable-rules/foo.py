from durable.lang import *

with ruleset('attributes'):
    @when_all(pri(3), m.amount < 300)
    def first_detect(c):
        print('attributes P3 ->{0}'.format(c.m.amount))
        
    @when_all(pri(2), m.amount < 200)
    def second_detect(c):
        print('attributes P2 ->{0}'.format(c.m.amount))
        
    @when_all(pri(1), m.amount < 100)
    def third_detect(c):
        print('attributes P1 ->{0}'.format(c.m.amount))
                
assert_fact('attributes', { 'amount': 50 })
print('---')
assert_fact('attributes', { 'amount': 150 })
print('---')
assert_fact('attributes', { 'amount': 250 })