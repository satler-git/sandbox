import           Acme.Missiles
import           Acme.RealWorld

main :: IO ()
main = do
    -- Save the current state of the universe
    world_as_we_know_it <- getWorld

    -- Cause serious international side effects
    launchMissiles

    -- After realizing that was a terrible, terrible mistake, restore the
    -- pre-war state of the universe.
    putWorld world_as_we_know_it

