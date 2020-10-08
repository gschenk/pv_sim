# Crate Meter

This simulates the electric power meter of a household. It returns
the instantaneous power the consumer draws.

# Crate PV
This simulates a photovoltaics system including including interface to its
consumer and the power grid.

# Crate Solarize
Solarize simulates the instantaneous insolation. It takes date [day number],
local solar time [s], and decimal latitude. It returns instantaneous
insolation [W/m^2] and zenith angle _z_ [rad] and azimuth angle _a_ [rad].

Zenith angle is calculated by: [1]

   cos z =sin l sin +cosϕ cos d cos h

where:
- latitude _l_ in rad
- declination _d_ (date)
- hour angle _h_ (0 at noon, 15 degree per hour, `15 * 60^(-3) * pi` [1/s])

The suns' azimuth equals the hour angle for afternoons in the the northern
hemisphere.  Trivial corrections yield all other cases.

##Approximations:
- the sun is a considered a point source: at `cos z <= 0` there is no insolation
- no atmospheric effects but attenuation
- air-mass is approximated by `min(sec z, 40)`
  - air mass is relative attenuation compared to horizon
  - plane parallel approximation
  - cut off at 40 to address earths curvature
  - solar panel peak power is usually give for 1.5 air mass
- no weather effects, its always blue skies
- sea level
- local solar time is approximated by local standard time
- every year is a leap year
- declination angle is approximated by `d_0 * cos(2 * pi * (n+10)/366)`
  - `d_0 = 0.4093` declination at the northern winter solstice 


[1] http://www.atmos.albany.edu/facstaff/brose/classes/ATM623_Spring2015/Notes/Lectures/Lecture11%20--%20Insolation.html
