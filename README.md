# Farmhand

Provides data needed to track yield farming returns in sheets.

```
           ______
      _.-"`      `"-._
    .'__ .. ._ .  .   '.
   / |__ /\ |_)|\/|     \
  /  |  /``\| \|  |      \
 ;                    _   ;
 |        |_| /\ |\ || \  |
 |     _. | |/``\| \||_/  |
 ;    /__`A   ,_          ;
  \   |= |;._.}{__       /
_.-""-|.' # '. `  `.-"{}<._
      / 1938  \     \  x   `"
 ----/         \_.-'|--X----
 -=_ |         |    |- X.  =_
- __ |_________|_.-'|_X-X##
jgs `'-._|_|;:;_.-'` '::.  `"-
 .:;.      .:.   ::.     '::.
```

## Ideas
* Simple APY is an extrapolation of earnings since start. In other words, take the value at t=0, take the value at t=now, and draw a line through both points to t=1y. We probably get a more accurate prediction by including data of each days value, i.e. a trend line.
