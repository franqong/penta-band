#pragma once

#include <QWidget>

class QHBoxLayout;
class QLabel;
class QPushButton;

class Header : public QWidget
{
    Q_OBJECT

public:
    explicit Header(QWidget *parent = nullptr);
    ~Header();

private:
    QWidget *backgroundWidget;
    QHBoxLayout *mainLayout;

    // Left
    QPushButton *writeButton;
    QPushButton *composeButton;
    QPushButton *recordButton;

    // Center
    QLabel *logoLabel;

    // Right
    QLabel *avatarLabel;
    QLabel *usernameLabel;
};
