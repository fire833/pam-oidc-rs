/*
*	Copyright (C) 2023 Kendall Tauser
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

		_, e = openid.NewClient(ctx, "pam-client", &openid.ClientArgs{
			RealmId:    demo.Realm,
			AccessType: pulumi.String("BEARER-ONLY"),
			Name:       pulumi.String("pam_local"),
			// Please do not use these credentials for anything but testing!
			ClientId:     pulumi.String("pam_local"),
			ClientSecret: pulumi.String("verybadsecret"),
		},
			pulumi.DependsOn([]pulumi.Resource{demo}))
		if e != nil {
			return e
		}

		// This user is only for demo purpose, please do not use in production.
		_, e = keycloak.NewUser(ctx, "demouser", &keycloak.UserArgs{
			RealmId:   demo.Realm,
			Enabled:   pulumi.Bool(true),
			Username:  pulumi.String("demouser"),
			FirstName: pulumi.String("John"),
			LastName:  pulumi.String("Doe"),
			InitialPassword: keycloak.UserInitialPasswordArgs{
				Value:     pulumi.String("pass"),
				Temporary: pulumi.Bool(false),
			},
			Email:         pulumi.String("john@email.com"),
			EmailVerified: pulumi.Bool(true),
		},
			pulumi.DependsOn([]pulumi.Resource{demo}))
		if e != nil {
			return e
		}

		return nil
	})
}
