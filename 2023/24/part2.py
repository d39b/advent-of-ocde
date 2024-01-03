from sympy import Symbol 
from sympy import solve_poly_system

x = Symbol("x")
y = Symbol("y")
z = Symbol("z")
vx = Symbol("vx")
vy = Symbol("vy")
vz = Symbol("vz")

t0 = Symbol("t0")
t1 = Symbol("t1")
t2 = Symbol("t2")

x0, y0, z0, vx0, vy0, vz0 = 152594199160345, 147562599184759, 291883234654893, 229, 220, -31
x1, y1, z1, vx1, vy1, vz1 = 181402578613976, 206158696386036, 294595238970734, 179, 99, -32
x2, y2, z2, vx2, vy2, vz2 = 306345582484815, 290719456201785, 306246299945991, -19, -64, -43
 
eq1 = x + vx*t0 - x0 - vx0*t0
eq2 = y + vy*t0 - y0 - vy0*t0
eq3 = z + vz*t0 - z0 - vz0*t0

eq4 = x + vx*t1 - x1 - vx1*t1
eq5 = y + vy*t1 - y1 - vy1*t1
eq6 = z + vz*t1 - z1 - vz1*t1

eq7 = x + vx*t2 - x2 - vx2*t2
eq8 = y + vy*t2 - y2 - vy2*t2
eq9 = z + vz*t2 - z2 - vz2*t2

equations = [eq1, eq2, eq3, eq4, eq5, eq6, eq7, eq8, eq9]

result = solve_poly_system(equations, *([x, y, z, vx, vy, vz, t0, t1, t2]))
print(result[0][0] + result[0][1] + result[0][2])
