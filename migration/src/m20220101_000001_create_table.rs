use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Event {
    Table,
    Id,
    DiscordServerId,
    DiscordCategoryId,
    DiscordMainChannelId,
    Name,
}

#[derive(Iden)]
enum Team {
    Table,
    Id,
    DiscordChannelId,
    EventId,
}

#[derive(Iden)]
enum Participant {
    Table,
    Id,
    DiscordId,
    TeamId,
    EventId,
}

#[derive(Iden)]
enum Submission {
    Table,
    Id,
    SubmissionData,
    ParticipantId,
}

const FK_TEAM_EVENT: &str = "fk_team_event";
const FK_PARTICIPANT_TEAM: &str = "fk_participant_team";
const FK_PARTICIPANT_EVENT: &str = "fk_participant_event";
const FK_SUBMISSION_PARTICIPANT: &str = "fk_submission_participant";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Table for event
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::DiscordServerId).string().not_null())
                    .col(ColumnDef::new(Event::DiscordCategoryId).string().not_null())
                    .col(
                        ColumnDef::new(Event::DiscordMainChannelId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Event::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Table for team
        manager
            .create_table(
                Table::create()
                    .table(Team::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Team::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Team::DiscordChannelId).string().not_null())
                    .col(ColumnDef::new(Team::EventId).integer().not_null())
                    // Create a foreign key to from team to event
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_TEAM_EVENT)
                            .from(Team::Table, Team::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Table for participant
        manager
            .create_table(
                Table::create()
                    .table(Participant::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Participant::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Participant::DiscordId).string().not_null())
                    .col(ColumnDef::new(Participant::TeamId).string())
                    .col(ColumnDef::new(Participant::EventId).string().not_null())
                    // Create a foreign key to from participant to team
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_PARTICIPANT_TEAM)
                            .from(Participant::Table, Participant::TeamId)
                            .to(Team::Table, Team::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // Create a foreign key to from participant to event
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_PARTICIPANT_EVENT)
                            .from(Participant::Table, Participant::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Table for submission
        manager
            .create_table(
                Table::create()
                    .table(Submission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Submission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Submission::SubmissionData)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Submission::ParticipantId)
                            .string()
                            .not_null(),
                    )
                    // Create a foreign key to from submission to participant
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_SUBMISSION_PARTICIPANT)
                            .from(Submission::Table, Submission::ParticipantId)
                            .to(Participant::Table, Participant::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the foreign key from submission to participant
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_SUBMISSION_PARTICIPANT)
                    .table(Submission::Table)
                    .to_owned(),
            )
            .await?;

        // Drop the foreign key from team to event
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_TEAM_EVENT)
                    .table(Team::Table)
                    .to_owned(),
            )
            .await?;

        // Drop the foreign key from participant to team
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_PARTICIPANT_TEAM)
                    .table(Participant::Table)
                    .to_owned(),
            )
            .await?;

        // Drop the foreign key from participant to event
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_PARTICIPANT_EVENT)
                    .table(Participant::Table)
                    .to_owned(),
            )
            .await?;

        // Drop the table for Event
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;

        // Drop the table for Team
        manager
            .drop_table(Table::drop().table(Team::Table).to_owned())
            .await?;

        // Drop the table for Participant
        manager
            .drop_table(Table::drop().table(Participant::Table).to_owned())
            .await?;

        // Drop the table for Submission
        manager
            .drop_table(Table::drop().table(Submission::Table).to_owned())
            .await?;

        Ok(())
    }
}
