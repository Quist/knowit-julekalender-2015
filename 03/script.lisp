(defun is-leap-year (year)
    (cond
        ((/= (mod year 4) 0) 0)
        ((/= (mod year 100) 0) 1)
        ((/= (mod year 400)0) 0)
        (1)
    )
)

(defun days-in-year(year)
    (if (= (is-leap-year year) 1)
        366
        365
    )
)

(defun days-between(start end days)
    (if (= start end)
    days
    (days-between
         (+ start 1)
         end
         (+ days (days-in-year start))
    ))
)

(defun calculate()
    (floor
        (days-between 1 2017 256)
        (* 364 7)
    )
)

(calculate)
