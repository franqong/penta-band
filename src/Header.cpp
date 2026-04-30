#include "Header.h"

#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QIcon>
#include <QPixmap>

Header::Header(QWidget *parent)
    : QWidget(parent)
{
    this->setFixedHeight(60);

    // This is the main layout for the Header widget itself
    QVBoxLayout *containerLayout = new QVBoxLayout(this);
    containerLayout->setContentsMargins(0, 0, 0, 0);

    // This widget will be our solid background
    backgroundWidget = new QWidget(this);
    backgroundWidget->setStyleSheet("background-color: #131315; border-bottom: 1px solid #676767;");
    
    // Add the background to the main container, making it fill the space
    containerLayout->addWidget(backgroundWidget);

    // The mainLayout now belongs to the backgroundWidget
    mainLayout = new QHBoxLayout(backgroundWidget);
    mainLayout->setContentsMargins(20, 0, 20, 0);
    mainLayout->setSpacing(20);

    // --- Left Section ---
    QHBoxLayout *leftLayout = new QHBoxLayout();
    leftLayout->setSpacing(15);

    writeButton = new QPushButton(QIcon(":/icons/write.svg"), "");
    composeButton = new QPushButton(QIcon(":/icons/compose.svg"), "");
    recordButton = new QPushButton(QIcon(":/icons/record.svg"), "");

    for (QPushButton* btn : {writeButton, composeButton, recordButton}) {
        btn->setFixedSize(40, 40);
        btn->setIconSize(QSize(24, 24));
        // Set border to none and background to transparent so it uses the parent's background
        btn->setStyleSheet("QPushButton { border: none; background-color: transparent; border-radius: 20px; } QPushButton:hover { background-color: #3a3a3e; }");
        leftLayout->addWidget(btn);
    }
    
    // --- Center Section ---
    logoLabel = new QLabel(this);
    QPixmap logoPixmap(":/images/penta-band.svg");
    logoLabel->setPixmap(logoPixmap.scaled(150, 40, Qt::KeepAspectRatio, Qt::SmoothTransformation));
    logoLabel->setAlignment(Qt::AlignCenter);

    // --- Right Section ---
    QHBoxLayout *rightLayout = new QHBoxLayout();
    rightLayout->setSpacing(10);
    rightLayout->setAlignment(Qt::AlignRight);

    avatarLabel = new QLabel(this);
    QPixmap avatarPixmap(":/icons/avatar.svg");
    avatarLabel->setPixmap(avatarPixmap.scaled(32, 32, Qt::KeepAspectRatio, Qt::SmoothTransformation));
    
    usernameLabel = new QLabel("Username", this);
    usernameLabel->setStyleSheet("color: #ffffff; font-size: 14px;");
    
    rightLayout->addWidget(avatarLabel);
    rightLayout->addWidget(usernameLabel);
    // Add a fixed spacer to balance the width with the leftLayout
    rightLayout->addSpacerItem(new QSpacerItem(43, 0, QSizePolicy::Fixed, QSizePolicy::Minimum));

    // --- Assemble Layout ---
    mainLayout->addLayout(leftLayout);
    mainLayout->addStretch();
    mainLayout->addWidget(logoLabel);
    mainLayout->addStretch();
    mainLayout->addLayout(rightLayout);

    // Set the layout for the background widget
    backgroundWidget->setLayout(mainLayout);
}

Header::~Header()
{
    // Qt manages child widgets memory, so no need to delete them manually.
}
