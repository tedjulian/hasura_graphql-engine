{-# LANGUAGE TemplateHaskell #-}

module Hasura.RQL.Types.Roles
  ( DropInheritedRole (..),
    InheritedRole,
    ParentRoles (..),
    Role (..),
  )
where

import Autodocodec (HasCodec (codec), dimapCodec, requiredField')
import Autodocodec qualified as AC
import Autodocodec.Extended (hashSetCodec)
import Data.Aeson
import Data.Aeson.Casing
import Data.Aeson.TH
import Hasura.Prelude
import Hasura.Session

newtype ParentRoles = ParentRoles {_unParentRoles :: HashSet RoleName}
  deriving (Show, Eq, ToJSON, FromJSON, Generic)

instance Hashable ParentRoles

instance HasCodec ParentRoles where
  codec = dimapCodec ParentRoles _unParentRoles hashSetCodec

-- | The `Role` type represents a role by
--   containing its name and the names of its parent roles.
--   This type is used externally in the `add_inherited_role`
--   metadata API and is also used internally
--   in the permission building
--   part of the schema cache building process
data Role = Role
  { _rRoleName :: RoleName,
    -- | set of the parent role names, in case of
    -- non-inherited roles it will be an empty set
    _rParentRoles :: ParentRoles
  }
  deriving (Show, Eq, Generic)

instance Hashable Role

instance HasCodec Role where
  codec =
    AC.object "Role" $
      Role
        <$> requiredField' "role_name" AC..= _rRoleName
        <*> requiredField' "role_set" AC..= _rParentRoles

instance ToJSON Role where
  toJSON (Role roleName parentRoles) =
    object
      [ "role_name" .= roleName,
        -- the key for parent roles is "role_set"
        -- in the JSON encoding of the `Role` type
        -- is because when this feature
        -- was introduced, it was added as "role_set"
        "role_set" .= parentRoles
      ]

instance FromJSON Role where
  parseJSON = withObject "Role" $ \o ->
    Role <$> o .: "role_name" <*> o .: "role_set"

type InheritedRole = Role

newtype DropInheritedRole = DropInheritedRole
  { _ddrRoleName :: RoleName
  }
  deriving (Show, Eq)

$(deriveJSON (aesonDrop 4 snakeCase) ''DropInheritedRole)
