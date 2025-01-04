/*
*	Copyright (C) 2025 Kendall Tauser
*
*	This program is free software; you can redistribute it and/or modify
*	it under the terms of the GNU General Public License as published by
*	the Free Software Foundation; either version 2 of the License, or
*	(at your option) any later version.
*
*	This program is distributed in the hope that it will be useful,
*	but WITHOUT ANY WARRANTY; without even the implied warranty of
*	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*	GNU General Public License for more details.
*
*	You should have received a copy of the GNU General Public License along
*	with this program; if not, write to the Free Software Foundation, Inc.,
*	51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

package main

import (
	"github.com/pulumi/pulumi-keycloak/sdk/v5/go/keycloak"
	"github.com/pulumi/pulumi-keycloak/sdk/v5/go/keycloak/openid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		demo, e := keycloak.NewRealm(ctx, "demorealm", &keycloak.RealmArgs{
			Realm:                       pulumi.String("demo"),
			DisplayName:                 pulumi.String("demo"),
			DisplayNameHtml:             pulumi.String("<b>demo</b>"),
			Enabled:                     pulumi.Bool(true),
			AccessCodeLifespan:          pulumi.String("1h"),
			AccessCodeLifespanLogin:     pulumi.String("2h"),
			LoginWithEmailAllowed:       pulumi.Bool(true),
			RegistrationEmailAsUsername: pulumi.Bool(true),
			LoginTheme:                  pulumi.String("keycloak"),
			AccountTheme:                pulumi.String("keycloak.v2"),
			AdminTheme:                  pulumi.String("keycloak.v2"),
			EmailTheme:                  pulumi.String("keycloak"),
		})
		if e != nil {
			return e
		}

		_, e = keycloak.NewRealmEvents(ctx, "demorealm-events", &keycloak.RealmEventsArgs{
			RealmId:            demo.Realm,
			AdminEventsEnabled: pulumi.Bool(true),
			EventsEnabled:      pulumi.Bool(true),
		},
			pulumi.DependsOn([]pulumi.Resource{demo}),
		)

		democlient, e := openid.NewClient(ctx, "pam-client", &openid.ClientArgs{
			RealmId:     demo.Realm,
			AccessType:  pulumi.String("CONFIDENTIAL"),
			Name:        pulumi.String("pamlocal"),
			Description: pulumi.String("Demo client for testing purposes. Please do not keep in production."),
			// Please do not use these credentials for anything but testing!
			ClientId:                  pulumi.String("pamlocal"),
			ClientSecret:              pulumi.String("verybadsecret"),
			ClientAuthenticatorType:   pulumi.String("client-secret"),
			DirectAccessGrantsEnabled: pulumi.Bool(true),
		},
			pulumi.DependsOn([]pulumi.Resource{demo}))
		if e != nil {
			return e
		}

		// This user is only for demo purpose, please do not use in production.
		demouser, e := keycloak.NewUser(ctx, "demouser", &keycloak.UserArgs{
			RealmId:   demo.ID(),
			Enabled:   pulumi.Bool(true),
			Username:  pulumi.String("john@t.co"),
			FirstName: pulumi.String("John"),
			LastName:  pulumi.String("Doe"),
			InitialPassword: keycloak.UserInitialPasswordArgs{
				Value:     pulumi.String("pass"),
				Temporary: pulumi.Bool(false),
			},
			Email:         pulumi.String("john@t.co"),
			EmailVerified: pulumi.Bool(true),
		},
			pulumi.DeleteBeforeReplace(true),
			pulumi.DependsOn([]pulumi.Resource{demo}))
		if e != nil {
			return e
		}

		demorole, e := keycloak.NewRole(ctx, "demo-role", &keycloak.RoleArgs{
			ClientId:    democlient.ID(),
			RealmId:     demo.ID(),
			Name:        pulumi.String("demo-role"),
			Description: pulumi.String("Demo role for testing. Please do not use in production."),
		},
			pulumi.DependsOn([]pulumi.Resource{demo, democlient}),
		)
		if e != nil {
			return e
		}

		_, e = keycloak.NewUserRoles(ctx, "demouser-pam", &keycloak.UserRolesArgs{
			RealmId: demo.ID(),
			UserId:  demouser.ID(),
			RoleIds: pulumi.StringArray{
				demorole.ID(),
			},
			Exhaustive: pulumi.Bool(false),
		},
			pulumi.DependsOn([]pulumi.Resource{demo, democlient}),
		)
		if e != nil {
			return e
		}

		return nil
	})
}
